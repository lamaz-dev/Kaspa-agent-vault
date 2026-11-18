# kaspa-agent-vault

Coffre de dépenses pour agents IA sur Kaspa : l'agent paie seul ses services, mais ne peut
jamais dépasser le budget fixé par son propriétaire. La limite est imposée par le réseau
Kaspa (covenant SilverScript), pas par un logiciel.

## Architecture

```
contracts/   le contrat AgentBudget (SilverScript), commenté en français
tests/       22 tests du contrat dans le moteur de script de Kaspa
cli/         outil en ligne de commande : déployer, payer, récupérer, démo x402 sur le testnet-10
spec/        spécification du profil x402 « covenant-budget » (paiement direct depuis le coffre)
testnet/     commandes à exécuter (commande.txt), journal des résultats, état public du coffre
.github/     exécution automatique sur le testnet via GitHub Actions
```

Prévu : `sdk/` (bibliothèque pour brancher un agent), `mcp-server/` (connecteur pour agents IA),
`demo-ui/` (démo web avec les transactions en direct).

## Le contrat (v0.3, prototype non audité)

- Plafond par paiement et budget rechargé en continu, plafonné (pas d'accumulation).
- Liste blanche de destinataires optionnelle.
- Couloirs parallèles pour les paiements simultanés, impossibles à combiner.
- Frais de transaction décomptés du budget (impossible de vider le coffre par les frais).
- Lignée unique suivie par le réseau (identifiant de covenant KIP-20) : pas de coffre cloné.
- Contrôle total du propriétaire : dépense hors budget, recharge dans la même lignée, récupération.

Modèle de menace : `spec/threat-model.md`. Prime aux bugs : `BUG-BOUNTY.md`.

## Profil x402 « covenant-budget »

Un agent paie un service x402 directement depuis son coffre. Le vendeur vérifie que le
coffre est authentique (recompilation du script), qu'il est solvable (budget lisible sur la
chaîne) et que la clé agent du coffre a autorisé cette requête, avant toute diffusion.
Démo : commande `demo-x402` (voir `testnet/journal.md`). Spécification : `spec/kaspa-covenant-budget-v0.md`.

## Lancer les tests

Copier `contracts/agent_budget.sil` et `tests/agent_budget_tests.rs` dans
`silverscript/silverscript-lang/tests/` (dépôt kaspanet/silverscript), puis :
`cargo test --release -p silverscript-lang --test agent_budget_tests`

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
