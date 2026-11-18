# Kaspa x402 — `covenant-budget` profile (draft v0)

> **Résumé en français.** Ce profil permet à un agent IA de payer un service x402
> directement depuis un coffre AgentBudget (covenant Kaspa), sans portefeuille
> intermédiaire. Le vendeur vérifie trois choses qu'aucun portefeuille d'agent classique
> ne permet : (1) le coffre est authentique (son script, recompilé à partir de paramètres
> publics, correspond à la sortie sur la chaîne), (2) le coffre est solvable (son budget
> restant, lisible dans son état, couvre le prix) et (3) le paiement est autorisé pour
> cette requête par la clé agent inscrite dans le coffre. Les limites de dépense restent
> imposées par le réseau Kaspa lui-même.

Status: draft, testnet-10 only. Companion to the Kaspa x402 `exact` binding
(`kaspa-exact-v2`). Reference implementation: `cli/src/x402.rs` in this repository.

## 1. Motivation

The `standard-native` profile of `kaspa-exact-v2` accepts only standard Schnorr P2PK
inputs with zero lock time. An agent that pays from a spending-limited covenant must
therefore move funds to a hot wallet first, where the limits no longer apply.

`covenant-budget` lets the payment come straight from an AgentBudget covenant, so the
spending limits (per-payment cap, continuously refilled budget, optional allowlist) are
enforced by consensus on every merchant payment, and adds merchant-side guarantees that
account-based agent wallets cannot offer.

## 2. Identifiers

```json
{
  "scheme": "exact",
  "network": "kaspa:testnet-10",
  "asset": "KAS",
  "extra": {
    "binding": "kaspa-covenant-budget-v0",
    "profile": "covenant-budget",
    "paymentFlow": "upfront",
    "finality": "accepted",
    "vaultTemplate": "agent-budget-v0.4"
  }
}
```

`vaultTemplate` names the covenant source the merchant recompiles against
(`contracts/agent_budget.sil`, version 0.4). A merchant MAY list several templates by
offering several `accepts` entries.

## 3. Transport

Unchanged x402 v2 HTTP transport: `402` with `PAYMENT-REQUIRED`, retry with
`PAYMENT-SIGNATURE`, success with `PAYMENT-RESPONSE`, all base64 JSON. Clients MUST NOT
follow redirects.

## 4. PaymentPayload

```json
{
  "x402Version": 2,
  "accepted": { "...": "the selected requirements, unchanged" },
  "payload": {
    "type": "covenant-budget-transaction",
    "profile": "covenant-budget",
    "transaction": "<hex borsh-encoded signed Kaspa transaction>",
    "vault": {
      "params": {
        "owner": "<x-only hex>", "agent": "<x-only hex>", "lane": 0,
        "capacity": 200000000, "refill_period": 36000, "max_per_payment": 50000000,
        "max_fee": 3000000, "use_allowlist": 0, "payees": ["<hex32>", "<hex32>", "<hex32>"],
        "tokens": 200000000, "last_daa": 626751782
      },
      "outpoint": { "txid": "<vault utxo txid>", "index": 0 }
    },
    "requestHash": "<hex blake2b-256(personal='x402cov-request', 'GET <url>')>",
    "authorization": {
      "version": "x402cov-auth-v0",
      "expiresAt": 1795012345,
      "signature": "<hex 64-byte Schnorr signature>"
    }
  }
}
```

## 5. Transaction shape

```text
input[0]  = the AgentBudget vault UTXO named by payload.vault.outpoint (entry `pay`)
output[0] = merchant payTo script, value = accepted.amount
output[1] = vault continuation (enforced by the covenant itself)
lock_time = the `nowDaa` claimed by the agent (proves elapsed time to the covenant)
payload   = empty (enforced by the covenant: no bridge instructions can ride along)
```

Exactly one input and two outputs; the covenant rejects anything else.

## 6. Verification (merchant)

The merchant MUST, in this order, and MUST NOT broadcast before steps 1–4 succeed:

1. **Terms.** `accepted` equals the offered requirements.
2. **Authentic vault.** Recompile the `vaultTemplate` source with `vault.params`;
   derive the P2SH script public key; the outpoint MUST exist on chain under that
   script. Any lie about the parameters (larger budget, other agent key) yields a
   different script and fails here.
3. **Solvency.** With `now` = current virtual DAA score minus a small margin:
   `amount ≤ max_per_payment` and `amount ≤ min(capacity, tokens + min(now − last_daa,
   refill_period) × capacity / refill_period)`.
4. **Transaction.** One input spending the vault outpoint; two outputs; output 0 pays
   exactly `amount` to `payTo`.
5. **Authorization.** `requestHash` matches the request; `expiresAt` not passed;
   `signature` verifies under the vault's `agent` key over
   `blake2b-256(personal='x402cov-auth-v0', "<txid>|<requestHash>|<payTo>|<amount>|<expiresAt>")`,
   where `txid` is recomputed from the transaction.
6. **Settlement.** Broadcast; serve the resource only once output 0 is accepted by the
   network (`finality: accepted`).

## 7. SettlementResponse

As in `kaspa-exact-v2`, with
`extensions.kaspa = { "binding": "kaspa-covenant-budget-v0", "profile": "covenant-budget",
"finality": "accepted", "paymentOutputIndex": 0 }` and `payer` = the vault address.

## 8. Security notes

- Steps 2–3 run before any broadcast: an insolvent or fake vault costs the merchant no
  on-chain interaction.
- The covenant is the last line of defence: even a merchant that skips step 3 cannot
  receive more than the vault allows.
- Concurrency: one vault UTXO pays one request at a time; use several lanes for
  parallel requests.
- The AgentBudget covenant is a prototype and has not been audited.

## 9. Assets

`vault.params.asset` names the budget asset and MUST equal `accepted.asset`.

| Asset | Status |
|---|---|
| `KAS` | Supported (template `agent-budget-v0.4`). |
| `KSD` | Reserved for Kaspa Dollar, the proposed KAS-collateralised stablecoin with a miner-voted price feed. A KSD vault template will compose with the KSD token covenant once its standard is published: same token covenant id in and out, budget counted in token units, a separate KAS UTXO for fees. |

Intermediate step under study: a KAS-settled vault whose budget is denominated in dollars,
converted at the miner-attested KAS price once that oracle is live.

## 10. Open questions for the Kaspa x402 maintainers

- Should `vaultTemplate` be identified by a template hash rather than a name?
- Should the transaction use the `kaspa-sdk-safe-json-v2.0.0` encoding instead of borsh?
- Should the authorization digest reuse the `kaspa-exact-v2` digest construction?
