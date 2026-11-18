# Prime aux bugs — AgentBudget v0.3 sur le testnet-10 (brouillon, pas encore annoncée)

## Périmètre

- Le contrat `contracts/agent_budget.sil` v0.3, déployé à l'adresse publiée ci-dessous sur le
  testnet-10 de Kaspa. Adresse et identifiant de lignée : *à publier lors de l'annonce*.
- Le vendeur de démo x402 « covenant-budget » (`cli/src/x402.rs`), dans la mesure où une
  attaque peut être reproduite contre lui.

Hors périmètre : le mainnet, les attaques contre GitHub ou l'infrastructure, les attaques à
51 %, l'ingénierie sociale, le déni de service par simple volume réseau.

## Conditions de victoire

1. **Dépasser le budget** : faire sortir du coffre plus que ce que les règles autorisent sur une
   période (montant + frais), sans la clé du propriétaire.
2. **Bloquer le propriétaire** : empêcher durablement la clé propriétaire de récupérer les fonds.
3. **Tromper le vendeur** : obtenir le contenu de démo avec un faux coffre, un coffre insolvable
   ou sans paiement accepté par le réseau.

La clé de l'agent pourra être publiée pour l'épreuve, afin de simuler un agent totalement compromis.

## Preuve demandée

- L'identifiant de la transaction (TxID) sur le testnet-10 ;
- la méthode de reproduction, pas à pas ;
- une ouverture d'issue privée sur le dépôt (ou par message), avant toute divulgation publique.

## Récompense

À définir avant l'annonce (montant symbolique en KAS, mention dans le dépôt et sur Kaspa France).

## Référence

Le modèle de menace (`spec/threat-model.md`) décrit les protections prévues et leurs tests.
