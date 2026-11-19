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

## 2026-11-18 15:06 UTC

```
$ vault-cli deployer
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Coffre déployé : 5 TKAS, budget 2 TKAS, lignée 7bbbb1a7947565a1619f5b5bd2ae36f2104602a728f3726849090843e46a2a7d
Transaction : https://tn10.kaspa.stream/transactions/1f009db42b7b6ba1e4ce8f001661362f98e012067e12062ea6d4176a0d61bec7

$ vault-cli pause 15
(pause de 15 s)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626786192
Solde du propriétaire : 1489.97 TKAS (1 UTXO)
Coffre : 5 TKAS, budget disponible ~2 TKAS (sortie 1f009db42b7b6ba1e4ce8f001661362f98e012067e12062ea6d4176a0d61bec7:0)

$ vault-cli payer 30000000
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Budget disponible : 2 TKAS, paiement demandé : 0.3 TKAS
Paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/14036a3999e2ce4e4293b9ee9c8fce39c0b1fa5b52dff2681037ad0275c26969

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli payer 100000000 --sans-verif
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Budget disponible : 1.69433333 TKAS, paiement demandé : 1 TKAS
Paiement REFUSÉ par le réseau : RPC Server (remote error) -> Rejected transaction 11525e3c5828db0121b4e5b1b66c1e8f57ae697c33d6f750f30045ecf22107dd: failed to verify the signature script: script ran, but verification failed
(code de sortie 2)

$ vault-cli recharger 200000000
Connecté au testnet-10 : wss://proton-10.kaspa.stream/kaspa/testnet-10/wrpc/borsh
Coffre rechargé de 2 TKAS (même lignée) : https://tn10.kaspa.stream/transactions/37405c178fb49ab45d9b99043d2acce053c8763afc43e5419d668dc59a649221

$ vault-cli pause 15
(pause de 15 s)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626787574
Solde du propriétaire : 1487.94 TKAS (1 UTXO)
Coffre : 6.685 TKAS, budget disponible ~1.74388888 TKAS (sortie 37405c178fb49ab45d9b99043d2acce053c8763afc43e5419d668dc59a649221:0)

$ vault-cli demo-x402
Vendeur de démo sur http://127.0.0.1:8402, paiements reçus sur kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh

=== Scénario 1 : achat normal, payé directement depuis le coffre ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] coffre authentique : script AgentBudget recompilé = sortie on-chain (6.685 TKAS), lignée 7bbbb1a7947565a1619f5b5bd2ae36f2104602a728f3726849090843e46a2a7d
[VENDEUR] solvable : budget restant 1.74772222 TKAS ≥ prix 0.2 TKAS
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/146d40a8b99aa5ad6336a81a72db6e822209bdcff074f48580da326181d721fe
[AGENT] ✅ contenu reçu : « Météo du réseau Kaspa : 10 blocs/s, mempool calme, frais bas. »
[AGENT] reçu de paiement : transaction 146d40a8b99aa5ad6336a81a72db6e822209bdcff074f48580da326181d721fe

=== Scénario 2 : prix au-dessus du plafond, le vendeur refuse avant toute diffusion ===

[AGENT] GET http://127.0.0.1:8402/rapport-premium
[VENDEUR] /rapport-premium : paiement requis (1 TKAS)
[AGENT] 402 reçu : 1 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] coffre authentique : script AgentBudget recompilé = sortie on-chain (6.47 TKAS), lignée 7bbbb1a7947565a1619f5b5bd2ae36f2104602a728f3726849090843e46a2a7d
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
Connecté au testnet-10 : wss://proton-10.kaspa.stream/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626788067
Solde du propriétaire : 1487.94 TKAS (1 UTXO)
Coffre : 6.47 TKAS, budget disponible ~1.55627777 TKAS (sortie 146d40a8b99aa5ad6336a81a72db6e822209bdcff074f48580da326181d721fe:1)

```

## 2026-11-18 16:02 UTC

```
$ vault-cli deployer
Connecté au testnet-10 : wss://proton-10.kaspa.stream/kaspa/testnet-10/wrpc/borsh
Coffre déployé : 5 TKAS, budget 2 TKAS, lignée b4f5c34ba99a8437a6169ed4583a903d613d8cf1e569cd12d276f47ca09cc768
Transaction : https://tn10.kaspa.stream/transactions/cb48263fa213a51c615c6ea103076eb1846c5d59e0e742eebe448a1bb433a0b2

$ vault-cli pause 15
(pause de 15 s)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626825747
Solde du propriétaire : 1482.925 TKAS (1 UTXO)
Coffre : 5 TKAS, budget disponible ~2 TKAS (sortie cb48263fa213a51c615c6ea103076eb1846c5d59e0e742eebe448a1bb433a0b2:0)

$ vault-cli sequestre-ouvrir 30000000 GET:/meteo-kaspa
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Commande « GET:/meteo-kaspa » : 0.3 TKAS mis en séquestre, délai 600 blocs (~60 s), contrat de 234 octets
Séquestre ouvert depuis le coffre : https://tn10.kaspa.stream/transactions/3734213a847d85d129baf85fbe6e54cdaf242cf98419e12378fff3883701804b

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli sequestre-reclamer Meteo-Kaspa:10-blocs/s --fausse-preuve
(fraude) FAUSSE preuve de livraison pour « GET:/meteo-kaspa » : résultat 5e9c3c77ae88f319
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Réclamation REFUSÉE par le réseau : RPC Server (remote error) -> Rejected transaction bf8f39647d8ac1f4e16fee9fac41a88329108f214ee4b0e8beeca9a72a6299e9: failed to verify the signature script: script ran, but verification failed

$ vault-cli sequestre-reclamer Meteo-Kaspa:10-blocs/s
Vérificateur : preuve de livraison pour « GET:/meteo-kaspa » : résultat 5e9c3c77ae88f319
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Vendeur payé contre preuve de livraison : https://tn10.kaspa.stream/transactions/bf8f39647d8ac1f4e16fee9fac41a88329108f214ee4b0e8beeca9a72a6299e9

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli sequestre-ouvrir 30000000 GET:/rapport
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Commande « GET:/rapport » : 0.3 TKAS mis en séquestre, délai 600 blocs (~60 s), contrat de 234 octets
Séquestre ouvert depuis le coffre : https://tn10.kaspa.stream/transactions/a42a636ccddd343412c01864d875fb77dae6df360da36d12ea8f9f45ab74d288

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli sequestre-rembourser
Connecté au testnet-10 : wss://electron-10.kaspa.blue/kaspa/testnet-10/wrpc/borsh
Remboursement REFUSÉ par le réseau : RPC Server (remote error) -> Rejected transaction 847c5b54e6b7e420c923b4ae251fa8704ab9311554022e12dd3c7234a33d6740: one of the transaction sequence locks conditions was not met

$ vault-cli pause 90
(pause de 90 s)

$ vault-cli sequestre-rembourser
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Séquestre remboursé au propriétaire (pas de livraison) : https://tn10.kaspa.stream/transactions/847c5b54e6b7e420c923b4ae251fa8704ab9311554022e12dd3c7234a33d6740

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli payer 5000000 --sans-verif --forcer-masse
(refus local KIP-9 désactivé : le réseau tranchera)
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Budget disponible : 1.5436111 TKAS, paiement demandé : 0.05 TKAS
Paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/df82ec4a5002f4780ee6b34912a174c8fe53593d9dce096357e9200e5c33d1cc

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli payer 1000000 --sans-verif --forcer-masse
(refus local KIP-9 désactivé : le réseau tranchera)
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Budget disponible : 1.49599998 TKAS, paiement demandé : 0.01 TKAS
Paiement REFUSÉ par le réseau : RPC Server (remote error) -> Rejected transaction 5e1ed0e1b8b832acfd6ba658d9b1a96d86554d7f45e3a6b77c7b13cf62752681: transaction storage mass of 1000055 is larger than max allowed size of 500000
(code de sortie 2)

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626829751
Solde du propriétaire : 1483.21 TKAS (2 UTXO)
Coffre : 4.305 TKAS, budget disponible ~1.51533332 TKAS (sortie df82ec4a5002f4780ee6b34912a174c8fe53593d9dce096357e9200e5c33d1cc:1)

```

## 2026-11-18 17:11 UTC

```
$ vault-cli etat
Connecté au testnet-10 : wss://alpha-10.kaspa.stream/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626873860
Solde du propriétaire : 1483.21 TKAS (2 UTXO)
Coffre : 4.305 TKAS, budget disponible ~2 TKAS (sortie df82ec4a5002f4780ee6b34912a174c8fe53593d9dce096357e9200e5c33d1cc:1)

$ vault-cli sequestre-ouvrir 30000000 GET:/meteo-kaspa/commande-3
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
Commande « GET:/meteo-kaspa/commande-3 » : 0.3 TKAS mis en séquestre, délai 600 blocs (~60 s), contrat de 242 octets
Séquestre ouvert depuis le coffre : https://tn10.kaspa.stream/transactions/4df7a2659a0f62fc96ea037d01891c9aae18458ad48a9ffcd902e2d7fc097cef

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli sequestre-reclamer Meteo-Kaspa:10-blocs/s
Vérificateur : preuve de livraison pour « GET:/meteo-kaspa/commande-3 » : résultat 5e9c3c77ae88f319
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Vendeur payé contre preuve de livraison : https://tn10.kaspa.stream/transactions/0897a9588735a2ae4b3fdbe3b55f94c209676d5b3c7477756c14863245d3f5e7

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli demo-x402
Vendeur de démo sur http://127.0.0.1:8402, paiements reçus sur kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh

=== Scénario 1 : achat normal, payé directement depuis le coffre ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] /meteo-kaspa : nœud : RPC Server (remote error) -> WebSocket -> WebSocket is not connected
[AGENT] ❌ refusé par le vendeur : nœud : RPC Server (remote error) -> WebSocket -> WebSocket is not connected

=== Scénario 2 : prix au-dessus du plafond, le vendeur refuse avant toute diffusion ===

[AGENT] GET http://127.0.0.1:8402/rapport-premium
[VENDEUR] /rapport-premium : paiement requis (1 TKAS)
[AGENT] 402 reçu : 1 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] /rapport-premium : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)
[AGENT] ❌ refusé par le vendeur : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)

=== Scénario 3 : l'agent présente un faux coffre, le vendeur le démasque ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
[AGENT] (test) j'annonce un faux coffre avec un budget 100 fois plus grand
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] /meteo-kaspa : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne
[AGENT] ❌ refusé par le vendeur : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626875596
Solde du propriétaire : 1483.21 TKAS (2 UTXO)
Coffre : 3.99 TKAS, budget disponible ~1.78111111 TKAS (sortie 4df7a2659a0f62fc96ea037d01891c9aae18458ad48a9ffcd902e2d7fc097cef:1)

```

## 2026-11-18 17:16 UTC

```
$ vault-cli demo-x402
Vendeur de démo sur http://127.0.0.1:8402, paiements reçus sur kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh

=== Scénario 1 : achat normal, payé directement depuis le coffre ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] coffre authentique : script AgentBudget recompilé = sortie on-chain (3.99 TKAS), lignée b4f5c34ba99a8437a6169ed4583a903d613d8cf1e569cd12d276f47ca09cc768
[VENDEUR] solvable : budget restant 1.86294444 TKAS ≥ prix 0.2 TKAS
[VENDEUR] paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/d3b55eb966a3158edce5acb30ff125efc530480a88d46f225c703d0fb0cd4816
[AGENT] ✅ contenu reçu : « Météo du réseau Kaspa : 10 blocs/s, mempool calme, frais bas. »
[AGENT] reçu de paiement : transaction d3b55eb966a3158edce5acb30ff125efc530480a88d46f225c703d0fb0cd4816

=== Scénario 2 : prix au-dessus du plafond, le vendeur refuse avant toute diffusion ===

[AGENT] GET http://127.0.0.1:8402/rapport-premium
[VENDEUR] /rapport-premium : paiement requis (1 TKAS)
[AGENT] 402 reçu : 1 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] /rapport-premium : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)
[AGENT] ❌ refusé par le vendeur : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)

=== Scénario 3 : l'agent présente un faux coffre, le vendeur le démasque ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://quark-10.kaspa.red/kaspa/testnet-10/wrpc/borsh
[AGENT] (test) j'annonce un faux coffre avec un budget 100 fois plus grand
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] /meteo-kaspa : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne
[AGENT] ❌ refusé par le vendeur : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli etat
Connecté au testnet-10 : wss://alpha-10.kaspa.stream/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 626878031
Solde du propriétaire : 1483.21 TKAS (2 UTXO)
Coffre : 3.775 TKAS, budget disponible ~1.70138888 TKAS (sortie d3b55eb966a3158edce5acb30ff125efc530480a88d46f225c703d0fb0cd4816:1)

```

## 2026-11-19 05:02 UTC

```
$ vault-cli demo-x402
Vendeur de démo sur http://127.0.0.1:8402, paiements reçus sur kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50

=== Scénario 1 : achat normal, payé directement depuis le coffre ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://electron-10.kaspa.stream/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] coffre authentique : script AgentBudget recompilé = sortie on-chain (3.775 TKAS), lignée b4f5c34ba99a8437a6169ed4583a903d613d8cf1e569cd12d276f47ca09cc768
[VENDEUR] solvable : budget restant 2 TKAS ≥ prix 0.2 TKAS
[VENDEUR] paiement accepté par le réseau : https://tn10.kaspa.stream/transactions/0e5b28695826fa965509f966a1f860bde11454f5c55b0f64947f5c82a5b47a74
[AGENT] ✅ contenu reçu : « Météo du réseau Kaspa : 10 blocs/s, mempool calme, frais bas. »
[AGENT] reçu de paiement : transaction 0e5b28695826fa965509f966a1f860bde11454f5c55b0f64947f5c82a5b47a74

=== Scénario 2 : prix au-dessus du plafond, le vendeur refuse avant toute diffusion ===

[AGENT] GET http://127.0.0.1:8402/rapport-premium
[VENDEUR] /rapport-premium : paiement requis (1 TKAS)
[AGENT] 402 reçu : 1 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] /rapport-premium : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)
[AGENT] ❌ refusé par le vendeur : refusé avant diffusion : prix 1 TKAS > plafond par paiement du coffre (0.5 TKAS)

=== Scénario 3 : l'agent présente un faux coffre, le vendeur le démasque ===

[AGENT] GET http://127.0.0.1:8402/meteo-kaspa
[VENDEUR] /meteo-kaspa : paiement requis (0.2 TKAS)
[AGENT] 402 reçu : 0.2 TKAS à payer à kaspatest:qqx5j2k4p60tgyh734me2xzg8dd8x8s5sfh744wsty9sktx2wpsxcjd8msm50
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
[AGENT] (test) j'annonce un faux coffre avec un budget 100 fois plus grand
[AGENT] paiement signé depuis le coffre, envoi au vendeur
[VENDEUR] autorisation valide : signée par la clé agent du coffre pour cette requête
[VENDEUR] /meteo-kaspa : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne
[AGENT] ❌ refusé par le vendeur : coffre non authentique : aucun coffre AgentBudget avec ces paramètres n'existe sur la chaîne

$ vault-cli pause 10
(pause de 10 s)

$ vault-cli etat
Connecté au testnet-10 : wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh
Score DAA du réseau : 627297643
Solde du propriétaire : 1483.21 TKAS (2 UTXO)
Coffre : 3.56 TKAS, budget disponible ~1.80427777 TKAS (sortie 0e5b28695826fa965509f966a1f860bde11454f5c55b0f64947f5c82a5b47a74:1)

```
