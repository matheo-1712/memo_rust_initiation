# Chapitre 12 — Projet final : **Mémo**

Tu as construit l'application par morceaux à travers neuf jalons. Ce chapitre sert à **assembler,
finir, durcir** — et à faire de Mémo un outil que tu utiliseras vraiment.

C'est aussi le chapitre où l'on te lâche : moins de doc, plus de spécification. C'est le but.

---

## 1. Où tu en es

| Jalon | Chapitre | Ce que tu as construit |
|-------|----------|------------------------|
| 1/9 | 3 | `Card`, `CardState`, `Grade` — le domaine typé |
| 2/9 | 4 | `Library` : ajout, suppression, recherche, `MemoError` |
| 3/9 | 5 | découpage en modules, `lib.rs` + `main.rs`, tests |
| 4/9 | 6 | trait `Scheduler`, implémentations `Leitner` et `Sm2` |
| 5/9 | 7 | statistiques et files de révision en style itérateur |
| 6/9 | 8 | `ReviewSession<'a>`, motif « ids plutôt que références », `Cow` |
| 7/9 | 9 | dates réelles, JSON, sauvegarde atomique, import/export |
| 8/9 | 10 | l'interface clap complète et le mode révision |
| 9/9 | 11 | import parallèle |

Si un jalon est resté en plan, **c'est maintenant qu'on le finit**. L'assemblage ne pardonne pas
les trous.

---

## 2. Définition de « terminé »

Mémo est fini quand **tout** ceci est vrai :

**Fonctionnel**
- [ ] les 11 sous-commandes du chapitre 10 fonctionnent et sont documentées dans `--help` ;
- [ ] une session de révision complète modifie l'état des cartes et le persiste ;
- [ ] les deux algorithmes sont sélectionnables par `--algo` et donnent des plannings différents ;
- [ ] import/export en aller-retour sans perte ;
- [ ] les données survivent à un redémarrage, et à un Ctrl+C pendant l'écriture.

**Qualité**
- [ ] `cargo test` : **au moins 40 tests** au vert, dont des tests d'intégration ;
- [ ] `cargo clippy --all-targets -- -D warnings` : zéro avertissement ;
- [ ] `cargo fmt --check` : propre ;
- [ ] `cargo doc` : tous les items publics documentés, exemples exécutables sur les principaux ;
- [ ] **aucun `unwrap()`/`expect()`** hors tests et hors cas justifié par un commentaire ;
- [ ] aucune panique possible depuis une entrée utilisateur ou un fichier corrompu.

**Ergonomie**
- [ ] tous les messages d'erreur disent quoi faire ensuite ;
- [ ] la sortie est lisible sans couleur et jolie avec ;
- [ ] `memo` sans argument affiche l'aide et sort avec le code 2 ;
- [ ] un `README.md` dans `projet/memo/` : à quoi ça sert, installation, exemples, format de fichier.

**Git**
- [ ] l'historique raconte la construction (un commit par jalon au minimum), messages en impératif.

---

## 3. Les finitions qui font la différence

### a) Le `README.md` du projet

Écris-le comme si un inconnu tombait dessus : une phrase de description, un GIF ou un bloc de
sortie console, l'installation (`cargo install --path projet/memo`), 5 exemples, le format du
fichier de données, la licence. C'est l'exercice le plus sous-estimé du métier.

### b) `cargo install --path projet/memo`

Ça installe `memo` dans `~/.cargo/bin` : tu peux alors taper `memo review` depuis n'importe où.
C'est le moment où ton exercice devient un logiciel. Vérifie que `--data` par défaut pointe bien
vers le dossier utilisateur et pas vers le dossier courant.

### c) Un vrai jeu de données

Crée `decks/rust.txt` avec 50 cartes tirées de ce cours (les questions d'auto-évaluation sont un
excellent point de départ), importe-le, et **utilise Mémo pendant deux semaines**.

Tu découvriras des défauts d'ergonomie qu'aucune relecture de code ne révèle. C'est l'objectif
pédagogique caché de tout le cours : passer de « le code compile » à « le logiciel sert ».

### d) Durcissement

Essaie activement de casser ton programme :

| Attaque | Attendu |
|---------|---------|
| question de 10 000 caractères | tronquée à l'affichage, stockée entière |
| question avec `|`, `\n`, des emojis, de l'arabe | export/import sans perte |
| fichier JSON tronqué au milieu | erreur claire, pas de panique |
| fichier JSON avec `version: 999` | erreur claire |
| `--limit 0`, `--limit 999999` | comportement sensé |
| paquet inexistant | message qui liste les paquets existants |
| deux instances de `memo` en même temps | au pire, la dernière écriture gagne — jamais de corruption |
| entrée non-UTF-8 dans un fichier importé | erreur, pas de panique |

Ajoute un test pour chaque cas que tu as réussi à casser.

---

## 4. Bonus, par difficulté croissante

Choisis-en deux ou trois. Ce sont de vrais projets d'apprentissage, pas du remplissage.

**★ Cartes à trous.** Une carte `Le mot-clé {{move}} force la capture par valeur` génère une
question à trou. Extension du modèle de domaine, parsing léger.

**★ Tags et filtres.** `memo review --tag difficile --tag chapitre2`. Ça touche la sérialisation
(champ `#[serde(default)]`), la CLI et les filtres d'itérateurs.

**★ Heatmap d'activité.** Un calendrier des 12 dernières semaines en blocs Unicode, façon GitHub.
Historique des révisions à persister, dates à manipuler, alignement de tableau à soigner.

**★★ Undo.** `memo undo` annule la dernière opération. Deux conceptions possibles : un journal
d'événements (`Vec<Event>` rejoué depuis zéro) ou des instantanés. Choisis, argumente, implémente.
C'est un exercice d'architecture bien plus que de syntaxe.

**★★ Mode QCM.** Pour une carte donnée, propose 4 réponses dont 3 tirées d'autres cartes du paquet.
Tirage aléatoire reproductible avec une graine.

**★★ Statistiques de rétention.** Pourcentage de réussite par intervalle. Ça te dira si ton
implémentation de SM-2 est bien calibrée — et c'est le genre de mesure que font les vrais logiciels
de répétition espacée.

**★★★ Interface TUI avec `ratatui`.** Panneaux, navigation au clavier, barre de progression,
graphiques. `cargo add ratatui crossterm`. C'est un gros morceau (boucle d'événements, état d'UI,
redessin), extrêmement formateur, et le résultat est spectaculaire.

**★★★ Synchronisation.** Un petit serveur HTTP (`axum`) et une commande `memo sync`. Là tu entres
dans l'async, la sérialisation réseau, la fusion de modifications concurrentes (que faire si la même
carte a été révisée sur deux machines ?). Prévois un week-end.

---

## 5. Le point sur l'algorithme SM-2

Puisque c'est le cœur intelligent, voici la spécification complète pour que tu puisses vérifier ton
implémentation. (Elle t'a été donnée en partie au chapitre 6 ; la voici en entier, comme référence.)

**État d'une carte :** un intervalle `I` en jours, un facteur de facilité `EF`, un compteur de
répétitions `n`.

**À chaque révision, avec une note `q` :**

1. Convertis ta note (0–3) vers l'échelle SM-2 (0–5). Un choix raisonnable :
   `oubliée → 1`, `difficile → 3`, `correcte → 4`, `facile → 5`. **Documente ton choix**, il
   change complètement le comportement.
2. Si `q < 3` : `n = 0`, `I = 1` — la carte repart. `EF` est mis à jour quand même (voir 4).
3. Sinon : `n += 1`, et
   - `n == 1` → `I = 1`
   - `n == 2` → `I = 6`
   - `n > 2` → `I = arrondi(I_précédent × EF)`
4. `EF = EF + (0.1 − (5 − q) × (0.08 + (5 − q) × 0.02))`, borné en bas à `1.3`.
5. Nouvelle échéance = aujourd'hui + `I` jours.

**Propriétés à tester** (ce sont de bons tests, parce qu'ils portent sur des invariants, pas sur des
valeurs magiques) :

- `EF` ne descend jamais sous 1.3, quel que soit le nombre d'échecs ;
- une note « facile » donne toujours un intervalle ≥ à celui d'une note « correcte » ;
- l'intervalle est strictement croissant tant que `q ≥ 3` ;
- un échec remet l'intervalle à 1 sans réinitialiser `EF` ;
- l'échéance est toujours strictement postérieure à la date de révision ;
- une carte suspendue n'apparaît jamais dans `due_cards`, quelle que soit la date.

Une propriété fausse dans ton implémentation = des semaines de révisions gâchées.
C'est un exemple parfait de code où les tests ne sont pas de la bureaucratie.

---

## 6. Et après ?

Tu sais maintenant écrire du Rust utile. Les directions naturelles :

**Approfondir le langage**
- *The Rust Programming Language* — <https://doc.rust-lang.org/book/> (le lire **après** ce cours
  est bien plus profitable qu'avant : tout te parlera)
- *Rust by Example*, et surtout **Rustlings** (`rustlings` en ligne de commande) pour la mécanique
- *Rust for Rustaceans* (Jon Gjengset) quand tu voudras passer au niveau supérieur — traits avancés,
  variance, unsafe, API design. C'est le meilleur second livre.

**Explorer des domaines**
- Web backend : `axum` + `sqlx` + `tokio`
- CLI avancé / TUI : `ratatui`
- WebAssembly : `wasm-bindgen`, `leptos`
- Systèmes embarqués : `embassy`, `no_std`
- Jeux : `bevy`

**Pratiquer**
- Refais un projet que tu connais dans un autre langage, en Rust : la comparaison est instructive.
- Contribue à une crate que tu utilises — l'écosystème Rust est accueillant, et lire du code de
  qualité est le meilleur accélérateur.
- Advent of Code en Rust, en te forçant à ne pas `unwrap`.

**Le conseil qui compte le plus :** tu as maintenant un outil que tu utilises quotidiennement et
dont tu es l'auteur. Chaque fois qu'il te frustre, corrige-le. Un logiciel qu'on maintient pendant
un an apprend plus que dix tutoriels.

---

## ✅ Le vrai examen final

Reprends les six questions d'auto-évaluation de chacun des onze chapitres précédents.
Mets-les dans `decks/rust.txt`. Importe-les dans Mémo. Révise-les pendant un mois.

Si tu réponds à toutes sans hésiter, tu ne « connais pas un peu Rust » : tu sais programmer en Rust.

Bravo. 🦀
