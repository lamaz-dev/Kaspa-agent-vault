# Invariants du coffre AgentBudget v0.4 et du séquestre DeliveryEscrow v0.1

*Matrice destinée au pré-audit. Chaque invariant doit rester vrai après **n'importe quelle** transaction acceptée par le consensus Kaspa, y compris forgée par un agent totalement compromis. Colonne « Vérifié par » : tests actuels ; « Étape 2 » : tests à propriétés (proptest) et preuve SMT à venir.*

Notations : `V` valeur de l'UTXO coffre dépensé, `V'` valeur de sa continuation, `B(t)` budget disponible à l'instant `t` = `min(capacity, tokens + min(t − lastDaa, refillPeriod) × capacity / refillPeriod)`.

## Coffre (entrée `pay`, signée par l'agent)

| # | Invariant | Formulation | Vérifié par |
|---|---|---|---|
| I1 | Conservation : l'agent ne dépense jamais plus que son budget | `V − V' ≤ B(nowDaa)` (paiement + frais) | `agent_ne_peut_pas_depasser_le_budget`, `vidage_par_les_frais_refuse` |
| I2 | Plafond par paiement | `outputs[0].value ≤ maxPerPayment` | `plafond_par_paiement_respecte` |
| I3 | Frais bornés | `V − V' − outputs[0].value ≤ maxFee` | `frais_au_dessus_du_plafond_refuses` |
| I4 | État honnête | la continuation porte `tokens' = B(nowDaa) − (V − V')` et `lastDaa' = nowDaa` | `agent_ne_peut_pas_tricher_sur_l_etat` |
| I5 | Règles immuables | la continuation a exactement le script du coffre (mêmes clés, plafonds, période, liste blanche) ; seul l'état change | `agent_ne_peut_pas_changer_les_regles`, `agent_ne_peut_pas_detourner_le_reste` |
| I6 | Lignée unique (KIP-20) | la continuation porte le même identifiant de covenant que l'entrée ; une sortie hors lignée ne peut pas payer | `clone_sans_lignee_refuse`, `continuation_hors_lignee_refusee` |
| I7 | Temps borné | `lastDaa ≤ nowDaa ≤ tx.daa` : l'agent ne peut ni remonter le temps ni annoncer un instant futur | `agent_ne_peut_pas_annoncer_un_instant_futur` |
| I8 | Pas d'accumulation | `B(t) ≤ capacity` pour tout `t` | `le_budget_inutilise_ne_s_accumule_pas` |
| I9 | Isolation des couloirs | une transaction `pay` a exactement 1 entrée et 2 sorties | `impossible_de_combiner_deux_couloirs`, `sortie_supplementaire_refusee` |
| I10 | Pas d'instruction cachée | payload vide | `instructions_de_bridge_refusees` |
| I11 | Liste blanche | si activée, `blake2b(outputs[0].scriptPubKey) ∈ {payeeA, payeeB, payeeC}` | `liste_blanche_activee_inconnu_refuse` |

## Coffre (entrée `ownerControl`, signée par le propriétaire)

| # | Invariant | Formulation | Vérifié par |
|---|---|---|---|
| I12 | Non-blocage | pour tout état du coffre, il existe une transaction signée par le propriétaire seul et acceptée par le consensus, y compris pour un couloir sous le plancher KIP-9 (entrée d'appoint) | `proprietaire_garde_la_main`, `proprietaire_peut_ajouter_une_entree_pour_payer_les_frais`, `proprietaire_recupere_un_couloir_presque_vide` |
| I13 | Exclusivité | seule la clé du propriétaire ouvre `ownerControl` ; seule la clé de l'agent ouvre `pay` | `mauvaise_cle_refusee_pour_payer` |

## Séquestre DeliveryEscrow

| # | Invariant | Formulation | Vérifié par |
|---|---|---|---|
| E1 | Le vendeur n'est payé que contre preuve | `claim` exige une signature du vérificateur sur `blake2b(requestHash ‖ resultHash)` | `vendeur_paye_contre_preuve_de_livraison`, `preuve_signee_par_un_autre_refusee`, `preuve_d_une_autre_commande_refusee` |
| E2 | Destination figée | `claim` → adresse du vendeur, `refund` → adresse de remboursement, une seule sortie | `preuve_valide_mais_argent_detourne_refuse`, `remboursement_vers_un_autre_refuse` |
| E3 | Remboursement garanti | après `timeoutDaa` blocs, `refund` est toujours possible sans aucune signature | `remboursement_apres_delai`, `remboursement_avant_delai_refuse` |
| E4 | Frais bornés | sortie ≥ valeur du séquestre − `maxFee` | `frais_excessifs_refuses` |

Limite connue (R22) : après le délai, `claim` et `refund` restent tous deux possibles ; la première transaction acceptée gagne.

## Limites physiques (consensus, hors script)

| # | Limite | Valeur mesurée | Vérifié par |
|---|---|---|---|
| P1 | Masse de calcul d'un paiement | 3 578 g | `masses_d_un_paiement_reel_sous_le_plafond` |
| P2 | Paiement minimal (KIP-9) | ≈ 0,02 KAS (limite réseau 500 000 g, observée sur testnet-10) | `micro_paiement_refuse_par_kip9`, `calcul_identique_au_reseau` |
| P3 | Plancher de survie d'un couloir | ≈ 0,125 KAS | `plancher_de_survie` |
| P4 | Arithmétique | entiers signés 64 bits ; `capacity × refillPeriod < 2^63` imposé à la création | `check_params` |
