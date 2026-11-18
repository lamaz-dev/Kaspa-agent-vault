# Journal du testnet-10

Résultats des commandes exécutées par GitHub Actions.

## 2026-11-18 13:54 UTC

```
$ vault-cli adresses
Error: "VAULT_PHRASE trop courte (16 caractères minimum)"
(code de sortie 1)

$ vault-cli etat
Error: "VAULT_PHRASE trop courte (16 caractères minimum)"
(code de sortie 1)

```

## 2026-11-18 13:59 UTC

```
$ vault-cli adresses
Propriétaire (à alimenter au faucet) : kaspatest:qqxetp2qx96n6u2p90recfdzqhgrcyrra9m3tqutglwc09e3kfz6us6tvxvuv
Agent                                : kaspatest:qpxe4y9j78lagg26q4dq98daaq2j44t3zqugqy3tgraj0yvmm7xac2jwt9d7m
Service payé par l'agent             : kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626744395
Solde du propriétaire : 0 TKAS (0 UTXO)
Coffre : pas encore déployé

```

## 2026-11-18 14:08 UTC

```
$ vault-cli etat
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626749810
Solde du propriétaire : 1500 TKAS (1 UTXO)
Coffre : pas encore déployé

$ vault-cli deployer
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Error: RpcSubsystem("Rejected transaction 88abeaf92b54ddc3b6bff35c4e4c7929d7fcd6e045f4159b4aea1ddbc117ba2c: transaction 88abeaf92b54ddc3b6bff35c4e4c7929d7fcd6e045f4159b4aea1ddbc117ba2c is not standard: transaction has 30000 fees which is under the required amount of 204700 for compute mass 2047")
(code de sortie 1)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626750431
Solde du propriétaire : 1500 TKAS (1 UTXO)
Coffre : pas encore déployé

```

## 2026-11-18 14:11 UTC

```
$ vault-cli deployer
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Coffre déployé : 5 TKAS, budget 2 TKAS
Transaction : https://tn10.kaspa.stream/transactions/93d64cf17505f4c3b0491a212f18952ecf0ae3bb4fc2ae5450383daa37b2140a

$ vault-cli etat
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626751902
Solde du propriétaire : 1494.985 TKAS (1 UTXO)
Coffre : 5 TKAS, budget disponible ~2 TKAS (sortie 93d64cf17505f4c3b0491a212f18952ecf0ae3bb4fc2ae5450383daa37b2140a:0)

```

## 2026-11-18 14:21 UTC

```
$ vault-cli payer 30000000
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Budget disponible : 2 TKAS, paiement demandé : 0.3 TKAS
Paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/398a6851f76c02dc4da6cdf19261b7c3c8d9bff24d11889d994c3f905de0bc29

$ vault-cli payer 100000000 --sans-verif
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Budget disponible : 1.72144444 TKAS, paiement demandé : 1 TKAS
Paiement REFUSÉ par le réseau : RPC Server (remote error) -> Rejected transaction 45607fdbf3502728250c1aa2e4cdeb1ad3355ec41db3fe4032d5d994ce6f8299: failed to verify the signature script: script ran, but verification failed
(code de sortie 2)

$ vault-cli payer 50000000
Connecté au testnet-10 : wss://electron-10.kaspa.blue/kaspa/testnet-10/wrpc/borsh
Budget disponible : 1.72255555 TKAS, paiement demandé : 0.5 TKAS
Paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/3287b4c1dbedc2a37ec3ec260ef7143c5c0e54587d3027a1934f6bf2d00e2459

$ vault-cli payer 50000000
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Budget disponible : 1.22299999 TKAS, paiement demandé : 0.5 TKAS
Paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/ea64df6fb13b2283ed68209b36cc9a75d559d25f4d2cad6b8be1aff26342f646

$ vault-cli payer 50000000
Connecté au testnet-10 : wss://electron-10.kaspa.blue/kaspa/testnet-10/wrpc/borsh
Budget disponible : 0.72349999 TKAS, paiement demandé : 0.5 TKAS
Paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/1607a240186c6b40193e833dd9de53157983d41f543f353bbd965f65b4ccbbb9

$ vault-cli payer 50000000 --sans-verif
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Budget disponible : 0.24738887 TKAS, paiement demandé : 0.5 TKAS
Paiement REFUSÉ par le réseau : RPC Server (remote error) -> Rejected transaction e8e90f14f452d15cbd70cffef745baaa352da58caac714ae85825381972f203e: failed to verify the signature script: script ran, but verification failed
(code de sortie 2)

$ vault-cli etat
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626759490
Solde du propriétaire : 1494.985 TKAS (1 UTXO)
Coffre : 3.14 TKAS, budget disponible ~0.25827776 TKAS (sortie 1607a240186c6b40193e833dd9de53157983d41f543f353bbd965f65b4ccbbb9:1)

```

## 2026-11-18 14:48 UTC

```
$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626774290
Solde du propriétaire : 1494.985 TKAS (1 UTXO)
Coffre : 3.14 TKAS, budget disponible ~1.08049999 TKAS (sortie 1607a240186c6b40193e833dd9de53157983d41f543f353bbd965f65b4ccbbb9:1)

$ vault-cli demo-x402
Vendeur de démo sur http://127.0.0.1:8402, paiements reçus sur kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh

=== Scénario 1 : achat normal, payé directement depuis le coffre ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://electron-10.kaspa.blue/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] coffre authentique : script AgentBudget recompilé = sortie on-chain (3.14 TKAS)
[VENDEUR] solvable : budget restant 1.08327776 TKAS ≥ prix 0.2 TKAS
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/a7c6e85b5b002cc70a9b75e5d8828ff7dc4cc0a39c3ddffd184afbf573b79367
[AGENT] ✅ contenu reçu : « Météo du réseau Kaspa : 10 blocs/s, mempool calme, frais bas. »
[AGENT] reçu de paiement : transaction a7c6e85b5b002cc70a9b75e5d8828ff7dc4cc0a39c3ddffd184afbf573b79367

=== Scénario 2 : prix au-dessus du plafond, le vendeur refuse avant toute diffusion ===

[AGENT] GET http://127.0.0.1:8402/rapport-premium
[VENDEUR] /rapport-premium : paiement requis (1 TKAS)
[AGENT] 402 reçu : 1 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] coffre authentique : script AgentBudget recompilé = sortie on-chain (2.925 TKAS)
[VENDEUR] /rapport-premium : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)
[AGENT] ❌ refusé par le vendeur : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)

=== Scénario 3 : l'agent présente un faux coffre, le vendeur le démasque ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] (test) j'annonce un faux coffre avec un budget 100 fois plus grand
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] /meteo-kaspa : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne
[AGENT] ❌ refusé par le vendeur : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626774393
Solde du propriétaire : 1494.985 TKAS (1 UTXO)
Coffre : 2.925 TKAS, budget disponible ~0.8862222 TKAS (sortie a7c6e85b5b002cc70a9b75e5d8828ff7dc4cc0a39c3ddffd184afbf573b79367:1)

```
