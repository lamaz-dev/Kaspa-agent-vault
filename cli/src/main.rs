//! vault-cli — déploie et utilise le coffre AgentBudget sur le testnet-10 de Kaspa.
//!
//! Clés (testnet uniquement) lues dans les variables d'environnement :
//!   OWNER_SK, AGENT_SK : clés privées hexadécimales (32 octets). Jamais dans le dépôt.
//! État du coffre (public, sans secret) : fichier JSON, par défaut `testnet/etat-coffre.json`.
//!
//! Commandes :
//!   cles                  génère deux nouvelles clés de test (à ranger dans les secrets GitHub)
//!   adresses              affiche les adresses du propriétaire, de l'agent et du service payé
//!   etat                  état du réseau, solde du propriétaire, état du coffre
//!   deployer              crée le coffre (couloir 0) avec les fonds du propriétaire
//!   payer <sompi> [--sans-verif]   l'agent paie le service depuis le coffre
//!   recuperer             le propriétaire récupère tout le coffre

use std::{env, fs, time::Duration};

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::hashing::sighash::{SigHashReusedValuesUnsync, calc_schnorr_signature_hash};
use kaspa_consensus_core::hashing::sighash_type::SIG_HASH_ALL;
use kaspa_consensus_core::network::{NetworkId, NetworkType};
use kaspa_consensus_core::tx::{
    PopulatedTransaction, ScriptPublicKey, Transaction, TransactionId, TransactionInput, TransactionOutpoint, TransactionOutput,
    UtxoEntry,
};
use kaspa_txscript::script_builder::ScriptBuilder;
use kaspa_txscript::{EngineFlags, pay_to_address_script, pay_to_script_hash_script};
use kaspa_wrpc_client::prelude::*;
use secp256k1::{Keypair, Message};
use serde::{Deserialize, Serialize};
use silverscript_abi::{ArtifactValue, SilAbiArtifact, encode_contract_entry_sig_script};
use silverscript_lang::compiler::{CompileOptions, compile_to_sil_abi_artifact_with_options};

mod x402;

const SOURCE: &str = include_str!("../../contracts/agent_budget.sil");

// Réglages du coffre de test (en sompi : 1 KAS = 100 000 000 sompi).
const CAPACITY: i64 = 200_000_000; // 2 TKAS de budget
const REFILL: i64 = 36_000; // recharge complète en ~1 h (10 DAA/s)
const MAX_PER_PAYMENT: i64 = 50_000_000; // 0,5 TKAS max par paiement
const MAX_FEE: i64 = 3_000_000; // frais max autorisés par paiement (0,03 TKAS)
const FEE: u64 = 1_500_000; // frais par entrée (le testnet exige 100 sompi par unité de masse)
const FUND: u64 = 500_000_000; // 5 TKAS déposés dans le coffre
const LANE: i64 = 0;
const USE_ALLOWLIST: i64 = 0;
const DAA_MARGIN: u64 = 100; // marge sous le score DAA du réseau (~10 s)

#[derive(Serialize, Deserialize)]
pub struct Etat {
    pub txid: String,
    pub index: u32,
    pub valeur: u64,
    pub tokens: i64,
    pub last_daa: i64,
}

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

/// Clés de test : dérivées de la phrase secrète VAULT_PHRASE (secret GitHub choisi par le
/// propriétaire, jamais visible ailleurs), ou à défaut lues en hexadécimal dans OWNER_SK / AGENT_SK.
pub fn keypair_from_env(var: &str) -> Res<Keypair> {
    if let Ok(phrase) = env::var("VAULT_PHRASE") {
        let phrase = phrase.trim();
        if phrase.len() < 16 {
            return Err("VAULT_PHRASE trop courte (16 caractères minimum)".into());
        }
        let h = blake2b_simd::Params::new().hash_length(32).personal(b"vault-testnet10").key(var.as_bytes()).hash(phrase.as_bytes());
        return Ok(Keypair::from_seckey_slice(secp256k1::SECP256K1, h.as_bytes())?);
    }
    let hexkey = env::var(var).map_err(|_| format!("variable {var} absente (ou VAULT_PHRASE)"))?;
    let bytes = hex::decode(hexkey.trim())?;
    Ok(Keypair::from_seckey_slice(secp256k1::SECP256K1, &bytes)?)
}

pub fn xonly(k: &Keypair) -> Vec<u8> {
    k.x_only_public_key().0.serialize().to_vec()
}

pub fn address(k: &Keypair) -> Address {
    Address::new(Prefix::Testnet, Version::PubKey, &xonly(k))
}

/// Le « service » payé par l'agent : une adresse de test dérivée de la clé du propriétaire.
pub fn payee(owner: &Keypair) -> Keypair {
    let h = blake2b_simd::Params::new().hash_length(32).personal(b"vault-payee").hash(&owner.secret_bytes());
    Keypair::from_seckey_slice(secp256k1::SECP256K1, h.as_bytes()).unwrap()
}

/// Paramètres publics d'un coffre (tout ce qu'il faut pour recompiler son script).
#[derive(Clone, Serialize, Deserialize)]
pub struct VaultParams {
    pub owner: String, // clé publique x-only, hex
    pub agent: String,
    pub lane: i64,
    pub capacity: i64,
    pub refill_period: i64,
    pub max_per_payment: i64,
    pub max_fee: i64,
    pub use_allowlist: i64,
    pub payees: [String; 3],
    pub tokens: i64,
    pub last_daa: i64,
}

impl VaultParams {
    pub fn ours(owner: &Keypair, agent: &Keypair, tokens: i64, last_daa: i64) -> Self {
        let zero = hex::encode([0u8; 32]);
        VaultParams {
            owner: hex::encode(xonly(owner)),
            agent: hex::encode(xonly(agent)),
            lane: LANE,
            capacity: CAPACITY,
            refill_period: REFILL,
            max_per_payment: MAX_PER_PAYMENT,
            max_fee: MAX_FEE,
            use_allowlist: USE_ALLOWLIST,
            payees: [zero.clone(), zero.clone(), zero],
            tokens,
            last_daa,
        }
    }
    /// Budget disponible à l'instant `now` (même calcul que le contrat).
    pub fn available(&self, now: i64) -> i64 {
        let elapsed = (now - self.last_daa).clamp(0, self.refill_period);
        (self.tokens + elapsed * self.capacity / self.refill_period).min(self.capacity)
    }
}

pub fn compile_params(p: &VaultParams) -> Res<SilAbiArtifact> {
    let b = |h: &str| -> Res<ArtifactValue> { Ok(hex::decode(h)?.into()) };
    let args: Vec<ArtifactValue> = vec![
        b(&p.owner)?,
        b(&p.agent)?,
        p.lane.into(),
        p.capacity.into(),
        p.refill_period.into(),
        p.max_per_payment.into(),
        p.max_fee.into(),
        p.use_allowlist.into(),
        b(&p.payees[0])?,
        b(&p.payees[1])?,
        b(&p.payees[2])?,
        p.tokens.into(),
        p.last_daa.into(),
    ];
    Ok(compile_to_sil_abi_artifact_with_options(SOURCE, &args, CompileOptions::default())?)
}

fn compile(owner: &Keypair, agent: &Keypair, tokens: i64, last_daa: i64) -> Res<SilAbiArtifact> {
    compile_params(&VaultParams::ours(owner, agent, tokens, last_daa))
}

/// Construit et signe un paiement de l'agent depuis le coffre.
/// Renvoie la transaction, l'état suivant et la valeur restante dans le coffre.
pub fn build_pay(
    owner: &Keypair,
    agent: &Keypair,
    e: &Etat,
    amount: i64,
    payee_spk: ScriptPublicKey,
    now: i64,
) -> Res<(Transaction, Etat)> {
    let p = VaultParams::ours(owner, agent, e.tokens, e.last_daa);
    let available = p.available(now);
    let vault = compile_params(&p)?;
    let next_tokens = available - amount;
    let next = compile(owner, agent, next_tokens, now)?;
    let entry = UtxoEntry::new(e.valeur, vault_spk(&vault), 0, false, None);
    let txid_in = TransactionId::from_bytes(<[u8; 32]>::try_from(hex::decode(&e.txid)?.as_slice())?);
    let input = TransactionInput::new(TransactionOutpoint::new(txid_in, e.index), vec![], 0, 1);
    let rest = e.valeur - amount as u64 - FEE;
    let outputs = vec![TransactionOutput::new(amount as u64, payee_spk), TransactionOutput::new(rest, vault_spk(&next))];
    let mut tx = Transaction::new(0, vec![input], outputs, now as u64, Default::default(), 0, vec![]);
    let s = sign(&tx, &[entry], 0, agent);
    tx.inputs[0].signature_script = vault_sigscript(&vault, "pay", &[s.into(), amount.into(), now.into()])?;
    tx.finalize();
    let next_state = Etat { txid: tx.id().to_string(), index: 1, valeur: rest, tokens: next_tokens, last_daa: now };
    Ok((tx, next_state))
}

pub fn bytecode(a: &SilAbiArtifact) -> Vec<u8> {
    a.contracts.values().next().expect("un contrat").compiled.bytecode.clone()
}

pub fn vault_spk(a: &SilAbiArtifact) -> ScriptPublicKey {
    pay_to_script_hash_script(&bytecode(a))
}

pub fn sign(tx: &Transaction, entries: &[UtxoEntry], idx: usize, k: &Keypair) -> Vec<u8> {
    let reused = SigHashReusedValuesUnsync::new();
    let populated = PopulatedTransaction::new(tx, entries.to_vec());
    let h = calc_schnorr_signature_hash(&populated, idx, SIG_HASH_ALL, &reused);
    let sig = k.sign_schnorr(Message::from_digest_slice(h.as_bytes().as_slice()).unwrap());
    let mut s = sig.as_ref().to_vec();
    s.push(SIG_HASH_ALL.to_u8());
    s
}

pub fn vault_sigscript(a: &SilAbiArtifact, entry: &str, args: &[ArtifactValue]) -> Res<Vec<u8>> {
    let name = a.contracts.keys().next().expect("un contrat").clone();
    let mut ss = encode_contract_entry_sig_script(a, &name, entry, args)?;
    ss.extend(ScriptBuilder::with_flags(EngineFlags::default()).add_data(&bytecode(a))?.drain());
    Ok(ss)
}

fn state_path() -> String {
    env::var("ETAT").unwrap_or_else(|_| "testnet/etat-coffre.json".into())
}

pub fn load_state() -> Res<Etat> {
    Ok(serde_json::from_str(&fs::read_to_string(state_path()).map_err(|_| "aucun coffre déployé (fichier d'état absent)")?)?)
}

pub fn save_state(e: &Etat) -> Res<()> {
    let p = state_path();
    if let Some(dir) = std::path::Path::new(&p).parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(p, serde_json::to_string_pretty(e)?)?;
    Ok(())
}

pub async fn connect() -> Res<KaspaRpcClient> {
    let net = NetworkId::with_suffix(NetworkType::Testnet, 10);
    let client = KaspaRpcClient::new_with_args(WrpcEncoding::Borsh, None, Some(Resolver::default()), Some(net), None)?;
    let opts = ConnectOptions {
        block_async_connect: true,
        strategy: ConnectStrategy::Fallback,
        connect_timeout: Some(Duration::from_secs(30)),
        ..Default::default()
    };
    client.connect(Some(opts)).await?;
    println!("Connecté au testnet-10 : {}", client.url().unwrap_or_default());
    Ok(client)
}

pub async fn virtual_daa(c: &KaspaRpcClient) -> Res<u64> {
    Ok(c.get_block_dag_info().await?.virtual_daa_score)
}

pub fn explorer(txid: &str) -> String {
    format!("https://tn10.kaspa.stream/transactions/{txid}")
}

#[tokio::main]
async fn main() -> Res<()> {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("aide");

    if cmd == "cles" {
        for name in ["OWNER_SK", "AGENT_SK"] {
            let k = Keypair::new(secp256k1::SECP256K1, &mut secp256k1::rand::thread_rng());
            println!("{name}={}", hex::encode(k.secret_bytes()));
        }
        return Ok(());
    }

    let owner = keypair_from_env("OWNER_SK")?;
    let agent = keypair_from_env("AGENT_SK")?;
    let service = payee(&owner);

    match cmd {
        "adresses" => {
            println!("Propriétaire (à alimenter au faucet) : {}", address(&owner));
            println!("Agent                                : {}", address(&agent));
            println!("Service payé par l'agent             : {}", address(&service));
        }
        "etat" => {
            let c = connect().await?;
            let daa = virtual_daa(&c).await?;
            let utxos = c.get_utxos_by_addresses(vec![address(&owner)]).await?;
            let bal: u64 = utxos.iter().map(|u| u.utxo_entry.amount).sum();
            println!("Score DAA du réseau : {daa}");
            println!("Solde du propriétaire : {} TKAS ({} UTXO)", bal as f64 / 1e8, utxos.len());
            match load_state() {
                Ok(e) => {
                    let elapsed = (daa.saturating_sub(DAA_MARGIN) as i64 - e.last_daa).clamp(0, REFILL);
                    let avail = (e.tokens + elapsed * CAPACITY / REFILL).min(CAPACITY);
                    println!(
                        "Coffre : {} TKAS, budget disponible ~{} TKAS (sortie {}:{})",
                        e.valeur as f64 / 1e8,
                        avail as f64 / 1e8,
                        e.txid,
                        e.index
                    );
                }
                Err(_) => println!("Coffre : pas encore déployé"),
            }
            c.disconnect().await?;
        }
        "deployer" => {
            if load_state().is_ok() {
                return Err("un coffre est déjà déployé".into());
            }
            let c = connect().await?;
            let now = (virtual_daa(&c).await? - DAA_MARGIN) as i64;
            let utxos = c.get_utxos_by_addresses(vec![address(&owner)]).await?;
            let mut picked = vec![];
            let mut total = 0u64;
            for u in utxos.into_iter().take(20) {
                total += u.utxo_entry.amount;
                picked.push(u);
                if total >= FUND + 2 * FEE + 100_000_000 {
                    break;
                }
            }
            if total < FUND + FEE {
                return Err(format!("solde insuffisant : {} TKAS, il en faut {}", total as f64 / 1e8, (FUND + FEE) as f64 / 1e8).into());
            }
            let vault = compile(&owner, &agent, CAPACITY, now)?;
            let entries: Vec<UtxoEntry> = picked.iter().map(|u| u.utxo_entry.clone().into()).collect();
            let inputs: Vec<TransactionInput> =
                picked.iter().map(|u| TransactionInput::new(u.outpoint.clone().into(), vec![], 0, 1)).collect();
            let fee = FEE * picked.len() as u64;
            let mut outputs = vec![TransactionOutput::new(FUND, vault_spk(&vault))];
            let change = total - FUND - fee;
            if change > 0 {
                outputs.push(TransactionOutput::new(change, pay_to_address_script(&address(&owner))));
            }
            let mut tx = Transaction::new(0, inputs, outputs, 0, Default::default(), 0, vec![]);
            for i in 0..tx.inputs.len() {
                let s = sign(&tx, &entries, i, &owner);
                tx.inputs[i].signature_script = ScriptBuilder::new().add_data(&s)?.drain();
            }
            tx.finalize();
            let txid = c.submit_transaction((&tx).into(), false).await?;
            println!("Coffre déployé : {} TKAS, budget {} TKAS", FUND as f64 / 1e8, CAPACITY as f64 / 1e8);
            println!("Transaction : {}", explorer(&txid.to_string()));
            save_state(&Etat { txid: txid.to_string(), index: 0, valeur: FUND, tokens: CAPACITY, last_daa: now })?;
            c.disconnect().await?;
        }
        "payer" => {
            let amount: i64 = args.get(2).ok_or("montant en sompi manquant")?.parse()?;
            let no_check = args.iter().any(|a| a == "--sans-verif");
            let e = load_state()?;
            let c = connect().await?;
            let now = ((virtual_daa(&c).await? - DAA_MARGIN) as i64).max(e.last_daa);
            let elapsed = (now - e.last_daa).min(REFILL);
            let available = (e.tokens + elapsed * CAPACITY / REFILL).min(CAPACITY);
            println!("Budget disponible : {} TKAS, paiement demandé : {} TKAS", available as f64 / 1e8, amount as f64 / 1e8);
            if !no_check && (amount > available || amount > MAX_PER_PAYMENT) {
                return Err("refusé localement : dépasse le budget ou le plafond par paiement (--sans-verif pour laisser le réseau trancher)".into());
            }
            let (tx, next_state) = build_pay(&owner, &agent, &e, amount, pay_to_address_script(&address(&service)), now)?;
            match c.submit_transaction((&tx).into(), false).await {
                Ok(txid) => {
                    println!("Paiement accepté par le réseau : {}", explorer(&txid.to_string()));
                    save_state(&next_state)?;
                }
                Err(err) => {
                    println!("Paiement REFUSÉ par le réseau : {err}");
                    std::process::exit(2);
                }
            }
            c.disconnect().await?;
        }
        "recuperer" => {
            let e = load_state()?;
            let c = connect().await?;
            let vault = compile(&owner, &agent, e.tokens, e.last_daa)?;
            let entry = UtxoEntry::new(e.valeur, vault_spk(&vault), 0, false, None);
            let txid_in = TransactionId::from_bytes(<[u8; 32]>::try_from(hex::decode(&e.txid)?.as_slice())?);
            let input = TransactionInput::new(TransactionOutpoint::new(txid_in, e.index), vec![], 0, 1);
            let outputs = vec![TransactionOutput::new(e.valeur - FEE, pay_to_address_script(&address(&owner)))];
            let mut tx = Transaction::new(0, vec![input], outputs, 0, Default::default(), 0, vec![]);
            let s = sign(&tx, &[entry], 0, &owner);
            tx.inputs[0].signature_script = vault_sigscript(&vault, "ownerControl", &[s.into()])?;
            tx.finalize();
            let txid = c.submit_transaction((&tx).into(), false).await?;
            println!("Coffre récupéré par le propriétaire : {}", explorer(&txid.to_string()));
            fs::remove_file(state_path())?;
            c.disconnect().await?;
        }
        "demo-x402" => {
            x402::demo(&owner, &agent).await?;
        }
        _ => {
            println!("Commandes : cles | adresses | etat | deployer | payer <sompi> [--sans-verif] | recuperer");
        }
    }
    Ok(())
}
