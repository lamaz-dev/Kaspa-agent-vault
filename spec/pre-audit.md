# Dossier de pré-audit — kaspa-agent-vault

*Pour l'auditeur externe et les participants à la prime aux bugs. Tout ce qui est affirmé ici est vérifiable dans le dépôt, avec la commande ou le test indiqué.*

## 1. Périmètre

| Élément | Fichier | Taille | Statut |
|---|---|---|---|
| Coffre AgentBudget v0.4 | `contracts/agent_budget.sil` | 479 octets (paramètres de référence) | dans le périmètre |
| Séquestre DeliveryEscrow v0.1 | `contracts/delivery_escrow.sil` | 234 octets | dans le périmètre |
| Vérification côté vendeur (profil x402) | `cli/src/x402.rs`, fonction `verify_and_settle` | — | dans le périmètre (logiciel hors chaîne) |
| Outil de démonstration | `cli/src/main.rs` | — | hors périmètre, sauf construction des transactions |

Hors périmètre : voir `spec/threat-model.md` §5 et `BUG-BOUNTY.md`.

## 2. Ce qui est figé

| Élément | Valeur | Vérification |
|---|---|---|
| Compilateur SilverScript | révision `443d519e6688fa03d24c69432763b13e6abe4486` | `cli/Cargo.toml`, `cli/Cargo.lock` |
| Chaîne d'outils Rust | 1.97.0 | `cli/rust-toolchain.toml` |
| Dépendances | figées (`--locked`) | `cli/Cargo.lock` |
| Empreintes des sources et du bytecode | `contracts/EMPREINTES.txt` | recalculées et comparées à chaque envoi par la CI publique (`.github/workflows/ci.yml`) |

Reproduire : `cd cli && cargo build --release --locked && ./target/release/vault-cli empreintes | diff - ../contracts/EMPREINTES.txt`.

Le bytecode dépend des paramètres du coffre (clés, plafonds, état). Pour un coffre réel, le vendeur recompile les paramètres annoncés et compare le P2SH obtenu à l'UTXO sur la chaîne : il ne fait confiance à aucun bytecode transmis.

## 3. Schémas des transactions

Notations : `V` valeur du coffre, `P` paiement, `F` frais, `S` séquestre. Toutes les transactions sont en version 1, chaque entrée porte un budget de calcul (`compute budget`).

### Déployer (propriétaire)

```
entrées : UTXO P2PK du propriétaire (1 à 20), budget de calcul 15 chacune
sorties : [0] coffre  V   P2SH(AgentBudget, tokens=capacity, lastDaa=now)   covenant = genèse(entrée 0, sortie 0)
          [1] monnaie     P2PK propriétaire (facultative)
```
L'identifiant de lignée (KIP-20) est `covenant_id(outpoint de l'entrée 0, [(0, sortie 0)])`.

### Payer (agent, entrée `pay`)

```
entrée  : [0] coffre V, covenant C, budget de calcul 20
          signature : sig_agent (SIGHASH_ALL), amount, nowDaa
sorties : [0] paiement   P         vers le destinataire (liste blanche facultative)
          [1] continuation V-P-F   P2SH(AgentBudget, tokens', lastDaa=nowDaa)  covenant C, autorisée par l'entrée 0
verrou temporel : lockTime = score DAA, avec lockTime ≥ nowDaa
payload : vide
```
Règles du script : invariants I1 à I11 (`spec/invariants.md`).

### Payer un séquestre (agent, entrée `pay`)

Identique à « Payer », la sortie 0 étant `P2SH(DeliveryEscrow)` sans covenant (une seule unité de stockage).

### Contrôle du propriétaire (entrée `ownerControl`)

```
entrées : [0] coffre (signature du propriétaire), [1..n] UTXO P2PK du propriétaire (facultatifs)
sorties : libres
```
Utilisé pour récupérer le coffre (sortie unique vers le propriétaire), le recharger (continuation de même état et même lignée, valeur augmentée) ou valider une dépense hors budget. Aucune contrainte autre que la signature : c'est voulu (I12, non-blocage).

### Réclamer un séquestre (vendeur, entrée `claim`)

```
entrée  : [0] séquestre S, séquence 0, budget de calcul 15
          signature : preuve = Schnorr(vérificateur, blake2b(requestHash ‖ resultHash)), resultHash
sortie  : [0] S-F vers l'adresse du vendeur figée à la création, F ≤ maxFee
```
Pas de signature du vendeur : la destination est figée, n'importe qui peut diffuser la réclamation une fois la preuve publiée.

### Rembourser un séquestre (n'importe qui, entrée `refund`)

```
entrée  : [0] séquestre S, séquence = timeoutDaa (verrou relatif), budget de calcul 5
sortie  : [0] S-F vers l'adresse de remboursement figée à la création, F ≤ maxFee
```

## 4. Hypothèses sur le consensus Kaspa

| # | Hypothèse | Sur quoi elle repose |
|---|---|---|
| H1 | `tx.daa` (verrou temporel absolu) ne peut pas dépasser le score DAA réel : une transaction au verrou futur n'est pas acceptée | règle de finalité du consensus |
| H2 | `this.ageDaa` (verrou relatif via la séquence) n'est satisfait qu'après `timeoutDaa` blocs de maturité de l'UTXO | observé sur testnet-10 : « sequence locks conditions was not met » (essai 8) |
| H3 | Une sortie liée à un covenant n'est acceptée que si une entrée de la même lignée l'autorise (KIP-20) | test `continuation_hors_lignee_refusee` |
| H4 | L'identifiant de transaction exclut les scripts de signature : une signature réécrite en transit ne change pas l'identifiant | R20 ; observé sur testnet-10 : la fausse et la vraie réclamation ont le même identifiant |
| H5 | Arithmétique du script sur entiers signés 64 bits, dépassement = échec du script | garde `check_params` (P4) |
| H6 | Limite de masse de stockage 500 000 g, frais de relais calculés sur la masse de calcul et transitoire seulement | mesuré sur testnet-10 (essai 8) ; code du nœud |
| H7 | La cadence du réseau (10 blocs/s) peut changer : les durées comptées en blocs changent de durée réelle | R6 |

## 5. Couverture des tests

| Suite | Nombre | Commande |
|---|---|---|
| Contrat AgentBudget (moteur SilverScript) | 25 + mesure du budget de calcul | job `contrats` de la CI |
| Séquestre DeliveryEscrow | 8 + mesure du budget de calcul | job `contrats` de la CI |
| Masse KIP-9, séquestre, outil | 13 | `cd cli && cargo test --release` |
| Fuzzing d'état | 4 (dont test de contrôle du banc) | idem ; campagne longue `FUZZ_CASES=5000` |
| Essais réels sur testnet-10 | 8 essais | `testnet/journal.md` |

Couverture de branches du code de vérification du vendeur (`cargo llvm-cov`) : à produire avant le gel du code.

## 6. Questions ouvertes pour l'auditeur

1. Le calcul `elapsed * capacity / refillPeriod` arrondit à l'inférieur : la perte d'arrondi peut-elle être exploitée en paiements très rapprochés ? (le budget ne peut que perdre, jamais gagner, selon nos tests)
2. Course `claim` / `refund` après le délai (R22) : la parade côté vendeur (réclamer dès la preuve, délai minimal 600 blocs) est-elle suffisante ?
3. `ownerControl` sans restriction : un propriétaire distrait peut détruire sa propre lignée. Faut-il une entrée de recharge contrainte ?
