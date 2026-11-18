# kaspa-agent-vault

Coffre de dépenses pour agents IA sur Kaspa : l'agent paie seul ses services,
mais ne peut jamais dépasser le budget fixé par son propriétaire. La limite est imposée
par le réseau Kaspa (covenant SilverScript), pas par un logiciel.

## État — v0.2 (prototype, non audité)

- Plafond par paiement, budget rechargé en continu et plafonné, liste blanche optionnelle.
- Couloirs parallèles pour les paiements simultanés.
- Contrôle total du propriétaire (dépense hors budget, récupération des fonds).
- 16 tests réussis dans le moteur de script de Kaspa. Pas encore déployé sur testnet.

## Contenu

- `contrat/agent_budget.sil` — le contrat, commenté en français.
- `tests/agent_budget_tests.rs` — les tests. Pour les lancer : copier ce fichier et `agent_budget.sil` dans
  `silverscript/silverscript-lang/tests/` (dépôt kaspanet/silverscript), puis
  `cargo test --release -p silverscript-lang --test agent_budget_tests`.

## Sécurité

Ne jamais déposer de clé privée ou de phrase secrète dans ce dépôt.
SilverScript n'a pas encore de version auditée : ne pas utiliser avec de vrais fonds.
