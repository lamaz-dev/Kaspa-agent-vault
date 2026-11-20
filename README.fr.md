# kaspa-agent-vault

*[English](README.md) · Français*

> ⚠️ **Prototype non audité, testnet-10 uniquement.** Ne déposez jamais de vrais KAS (mainnet) dans ces
> contrats. Le code est en revue publique avant audit : voir [`SECURITY.md`](SECURITY.md) pour signaler une
> faille en privé et [`BUG-BOUNTY.md`](BUG-BOUNTY.md) pour les règles.

**Statut**

- ✅ testnet-10
- ✅ borne de perte démontrée sur papier et vérifiée par tests automatisés contre le vrai moteur de Kaspa ([preuve](spec/borne-de-perte.md))
- ✅ reproductible en une commande : `./verify-proof.sh`
- ✅ CI automatique à chaque envoi, campagne longue de fuzzing chaque semaine
- ⚠️ pas encore relu de façon indépendante, non audité
- ⚠️ aucun déploiement mainnet : ne pas utiliser avec des fonds réels

Avis de sécurité : [faille de la limite par période en v0.4, corrigée en v0.5](spec/advisories/2026-11-20-v0.4-temps-mis-de-cote.md).

**Démonstrations testnet-10 (v0.5, 20 nov. 2026)** — journal complet : [`testnet/journal.md`](testnet/journal.md)

| Étape | Résultat | Transaction |
|---|---|---|
| Ancien coffre v0.4 rapatrié par son propriétaire | acceptée | [`7128d61c…`](https://tn10.kaspa.stream/transactions/7128d61c174426b7c0e445eca0d610def7f02ae673fce0a19793b80a8f6c04f4) |
| Coffre v0.5 créé (5 TKAS, budget 2 TKAS) | acceptée | [`0ce4a427…`](https://tn10.kaspa.stream/transactions/0ce4a427418bf0cb6d18fe23e806aa168a0b3ea4fdc42d6308d9fac6812a6c67) |
| 1. Achat de l'agent dans le budget (0,2 TKAS) | acceptée | [`e4e4e545…`](https://tn10.kaspa.stream/transactions/e4e4e545344e67272c35c85d01e10559b27435a3c4f095fdae885dd199d7e4d5) |
| 2. Prix au-dessus du plafond par achat (0,6 TKAS) | refusé par l'outil, rien n'est envoyé | — |
| 3. Agent compromis qui contourne l'outil et l'envoie quand même | **rejetée par le réseau** (`script ran, but verification failed`) ; une transaction rejetée n'arrive jamais sur la chaîne, son identifiant `23f56471…` n'a donc pas de page dans l'explorateur | — |
| 4. Gel par le propriétaire : tout revient sur son adresse | acceptée | [`fabc445d…`](https://tn10.kaspa.stream/transactions/fabc445d86fbabeea0431c22c8a21964f5f0469eb1241f94cc80121f5a04447f) |

>
> ⚠️ **Unaudited prototype, testnet-10 only.** Never deposit real (mainnet) KAS into these contracts.
> Public review before audit: report vulnerabilities privately as described in [`SECURITY.md`](SECURITY.md).

Coffre de dépenses pour agents IA sur Kaspa : l'agent paie seul ses services, mais ne peut
jamais dépasser le budget fixé par son propriétaire. La limite est imposée par le réseau
Kaspa (covenant SilverScript), pas par un logiciel.

## Architecture

```
contracts/   AgentBudget (coffre à budget) et DeliveryEscrow (paiement à la livraison), en SilverScript commenté
tests/       25 tests du coffre + 10 du séquestre, dans le moteur de script de Kaspa (outil : 21 tests dont fuzzing d'état)
cli/         outil en ligne de commande : déployer, payer, récupérer, démo x402 sur le testnet-10
spec/        spécification du profil x402 « covenant-budget » (paiement direct depuis le coffre)
testnet/     commandes à exécuter (commande.txt), journal des résultats, état public du coffre
.github/     exécution automatique sur le testnet via GitHub Actions
```

## Le coffre AgentBudget (v0.5, prototype non audité)

- Plafond par paiement et budget rechargé en continu, plafonné (pas d'accumulation).
- Liste blanche de destinataires optionnelle.
- Couloirs parallèles pour les paiements simultanés, impossibles à combiner.
- Frais de transaction décomptés du budget (impossible de vider le coffre par les frais).
- Lignée unique suivie par le réseau (identifiant de covenant KIP-20) : pas de coffre cloné.
- Paiements de l'agent sans données attachées : pas d'instructions de bridge cachées.
- Monnaie du budget en paramètre : KAS aujourd'hui, Kaspa Dollar (KSD) réservé.
- Contrôle total du propriétaire : dépense hors budget, recharge dans la même lignée, récupération.

Modèle de menace : `spec/threat-model.md`. Prime aux bugs : `BUG-BOUNTY.md`.

## Le séquestre DeliveryEscrow (v0.2, prototype)

Paiement à la livraison : le vendeur n'est payé que contre une preuve de livraison signée par
un vérificateur pour cette commande précise ; sans preuve dans le délai, l'argent revient au
propriétaire. Ni l'un ni l'autre ne peut détourner les fonds ni gonfler les frais.

## Profil x402 « covenant-budget »

Un agent paie un service x402 directement depuis son coffre. Le vendeur vérifie que le
coffre est authentique (recompilation du script), qu'il est solvable (budget lisible sur la
chaîne) et que la clé agent du coffre a autorisé cette requête, avant toute diffusion.
Démo : commande `demo-x402` (voir `testnet/journal.md`). Spécification : `spec/kaspa-covenant-budget-v0.md`.

## Lancer les tests

Voir la section « Reproduce » du [README anglais](README.md#reproduce) : tests de l'outil (dont fuzzing d'état), empreintes du bytecode, tests des contrats sur SilverScript figé.

## Testnet-10

Écrire les commandes dans `testnet/commande.txt` (`adresses`, `etat`, `deployer`,
`payer <sompi>`, `recuperer`) et pousser : GitHub Actions les exécute et écrit le
résultat dans `testnet/journal.md`. Les clés de test sont dérivées du secret
`VAULT_PHRASE` du dépôt.

## Sécurité

- Aucune clé, phrase secrète ou URL privée dans le dépôt : uniquement dans les secrets GitHub
  ou des fichiers `.env` ignorés.
- Clés dédiées à la démo, jamais réutilisées ailleurs.
- SilverScript n'a pas encore de version auditée : ne pas utiliser avec de vrais fonds.

## Licence

Le cœur ouvert (contrats SilverScript, spécifications `spec/`, outil de démonstration `cli/`) est publié sous licence ISC, la même que rusty-kaspa et SilverScript. Voir `LICENSE`.
