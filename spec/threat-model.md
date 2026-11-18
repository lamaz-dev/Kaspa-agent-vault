# Modèle de menace — AgentBudget v0.3 et profil x402 « covenant-budget »

Statut : brouillon, prototype non audité, testnet-10 uniquement. Ce document sert de cahier
des charges aux tests antagonistes et au périmètre de la prime aux bugs (`BUG-BOUNTY.md`).

## 1. Ce qu'on protège

| Bien | Propriétaire | Ce qu'on garantit |
|---|---|---|
| Fonds du coffre | Propriétaire | L'agent ne peut sortir que `capacity` d'un coup, puis au rythme de la recharge ; montant + frais comptés. |
| Contrôle du coffre | Propriétaire | La clé propriétaire peut toujours tout récupérer ou recharger, y compris des fonds envoyés hors lignée. |
| Revenu du vendeur | Vendeur | Le vendeur ne livre qu'après acceptation par le réseau d'un paiement venant d'un coffre authentique et solvable. |

## 2. Acteurs et hypothèses de confiance

| Acteur | Hypothèse |
|---|---|
| Propriétaire | De confiance. Sa clé est hors ligne ou sur son téléphone, jamais sur le serveur ni chez l'agent. |
| Agent IA | **Supposé totalement compromis** (injection de prompt, clé volée). Le coffre doit tenir quoi qu'il fasse. |
| Vendeur | Honnête mais exposé : peut être spammé ; ne doit jamais livrer sans paiement accepté. |
| Mineurs | Majorité honnête (hypothèse de consensus Kaspa). |
| Réseau | 10 blocs/s au moment de l'écriture. Voir R6. |

## 3. Scénario central : l'agent est totalement compromis

L'attaquant détient la clé agent. Ce qu'il peut faire, au maximum :

- dépenser `capacity` immédiatement (montant + frais), puis `capacity` par `refillPeriod` ;
- jamais plus de `maxPerPayment` par paiement ;
- vers n'importe quel destinataire si la liste blanche est désactivée (c'est un choix du propriétaire) ;
- rien de plus : ni plus vite, ni par les frais, ni via un clone, ni en combinant des couloirs.

Parade côté propriétaire : récupérer le coffre (`recuperer`) ; la clé agent volée devient inutile.

## 4. Risques et parades

| # | Risque | Parade | Vérification |
|---|---|---|---|
| R1 | Vider le coffre par les frais (paiements minuscules à frais élevés) | Montant + frais décomptés du budget ; frais ≤ `maxFee` | tests `vidage_par_les_frais_refuse`, `frais_au_dessus_du_plafond_refuses` |
| R2 | Coffre cloné : fonds envoyés à l'adresse du coffre = budget dupliqué | Lignée KIP-20 obligatoire pour `pay` ; continuation liée à la lignée | tests `clone_sans_lignee_refuse`, `continuation_hors_lignee_refusee` |
| R3 | Fonds bloqués par des envois parasites (poussière) à l'adresse | Les envois parasites sont des UTXO séparés : ils ne bloquent pas le coffre ; le propriétaire peut tout balayer | test `proprietaire_recupere_meme_un_clone` |
| R4 | Mentir sur le temps pour recharger plus vite | `nowDaa` borné par le verrou temporel (CLTV) : un instant futur est refusé par le réseau ; un instant passé réduit le budget | test `agent_ne_peut_pas_annoncer_un_instant_futur`, refus observé sur testnet |
| R5 | Accumuler le budget pendant l'inactivité | Recharge plafonnée à `capacity` | test `le_budget_inutilise_ne_s_accumule_pas` |
| R6 | **Changement de cadence du réseau** (10 → 20+ blocs/s) : la recharge, comptée en blocs, s'accélère d'autant en temps réel | Hypothèse opérationnelle documentée : migrer les coffres (recréer avec `refillPeriod` ajusté) lors d'un hard fork de cadence | à surveiller à chaque hard fork |
| R7 | Combiner deux couloirs dans une transaction | Une seule entrée, deux sorties exactement | test `impossible_de_combiner_deux_couloirs` |
| R8 | Dépassement d'entier dans le calcul de recharge | `elapsed` plafonné à `refillPeriod` ; contrainte `capacity × refillPeriod < 2^63` à vérifier à la création | à couvrir par du fuzzing |
| R9 | Vendeur trompé par un faux coffre | Recompilation des paramètres annoncés + présence on-chain + lignée | scénario 3 de la démo testnet |
| R10 | Vendeur livrant avant paiement (0-conf) | Livraison seulement après acceptation de la sortie par le réseau | démo testnet |
| R11 | Rejeu d'une autorisation x402 | Autorisation liée à l'identifiant de transaction (UTXO dépensé une seule fois), à la requête et à une expiration | revue de code |
| R12 | Déni de service sur le vendeur (recompilations en masse) | Cache des scripts recompilés ; vérifications bon marché d'abord | à compléter par une limite de débit |
| R13 | Injection via GitHub Actions | Déclenchement seulement sur la branche principale ; les demandes externes n'ont pas accès aux secrets ; clés de test seulement | à durcir : liste blanche des commandes |
| R14 | Agent qui paie un service légitime en boucle (injection de prompt) | Le budget borne la perte ; le futur serveur MCP ajoutera un coupe-circuit (débit par domaine, confirmation humaine au-delà d'un seuil) | à concevoir avec `mcp-server/` |
| R15 | Recharge du propriétaire pendant une dépense de l'agent | Les deux transactions dépensent le même UTXO : une seule est acceptée, l'autre est rejetée sans perte | à tester sur testnet |

## 5. Hors périmètre (v0.3)

- Sécurité de l'appareil du propriétaire.
- Erreurs du compilateur SilverScript lui-même (pas encore de version auditée).
- Attaque à 51 % du réseau Kaspa.
- Le standard kUSDT (v2).

## 6. Prochains tests antagonistes

- Fuzzing des paramètres et des valeurs limites (montants nuls, négatifs, maximum, `refillPeriod` = 1).
- Paiements concurrents sur un même couloir (R15).
- Coffres dont les paramètres frôlent la limite de dépassement (R8).
