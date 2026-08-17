# Apprendre Rust — de zéro à une vraie application

Bienvenue. Ce dépôt est un **cours complet de Rust en français**, organisé en 13 chapitres.
Chaque chapitre est un dossier avec un `README.md` qui contient :

- les **notions** à comprendre (avec des extraits de syntaxe pour illustrer, jamais la solution) ;
- les **pièges classiques** et comment lire les messages du compilateur ;
- des **exercices** variés, avec spécification précise, sorties attendues et critères de réussite ;
- à partir du chapitre 3, un **jalon** qui fait avancer l'application finale.

> **Règle du jeu :** je ne te donne jamais le code que tu dois écrire. Je te donne la doc,
> les signatures, le comportement attendu et les indices. Le code, c'est toi.

---

## L'application finale : **Mémo**

À la fin du cours, tu auras écrit **Mémo**, un logiciel de révision en ligne de commande
basé sur la **répétition espacée** (le principe d'Anki, en beaucoup plus simple).

Ce que Mémo saura faire :

```
$ memo add --deck rust "Que fait le mot-clé `move` ?" "Force la closure à prendre possession..."
Carte #42 ajoutée au paquet « rust ».

$ memo review --deck rust
Paquet « rust » — 7 cartes à réviser aujourd'hui

[1/7] Que fait le mot-clé `move` ?
      (Entrée pour révéler)
      > Force la closure à prendre possession des variables capturées.
      Ta réponse : (0) oubliée  (1) difficile  (2) correcte  (3) facile  > 2
      Prochaine révision dans 6 jours.
...
Session terminée : 7 cartes, 85 % de réussite, 1 min 12 s.

$ memo stats
Paquets : 3 | Cartes : 128 | À réviser demain : 14
rust     ████████████░░░░░░  62 cartes  (maturité 71 %)
anglais  ██████░░░░░░░░░░░░  38 cartes  (maturité 44 %)
```

Pourquoi ce projet ? Parce qu'il t'oblige à croiser **tout** ce qui fait Rust :
modélisation avec `struct`/`enum`, `Option`/`Result`, collections, traits, itérateurs,
gestion d'erreurs propre, lecture/écriture de fichiers, sérialisation JSON, arguments
de ligne de commande, tests, et même un peu de parallélisme. Et à la fin, tu t'en serviras
pour réviser… Rust.

---

## Le parcours

| # | Chapitre | Notions clés | Jalon Mémo |
|---|----------|--------------|------------|
| 00 | [Mise en route](cours/00-mise-en-route/README.md) | toolchain, cargo, rustfmt, clippy | — |
| 01 | [Les bases du langage](cours/01-bases-du-langage/README.md) | variables, types, fonctions, contrôle de flux | — |
| 02 | [Ownership et emprunts](cours/02-ownership-et-emprunts/README.md) | move, `&`, `&mut`, slices, `String` vs `&str` | — |
| 03 | [Structs, enums, pattern matching](cours/03-structs-enums-pattern-matching/README.md) | `struct`, `enum`, `match`, `impl` | Le type `Card` |
| 04 | [Collections et erreurs](cours/04-collections-et-erreurs/README.md) | `Vec`, `HashMap`, `Option`, `Result`, `?` | Le `Deck` |
| 05 | [Modules, tests et crates](cours/05-modules-tests-et-crates/README.md) | `mod`, visibilité, `#[test]`, dépendances | Découpage en modules |
| 06 | [Traits et génériques](cours/06-traits-et-generiques/README.md) | `trait`, `impl Trait`, `dyn`, bornes | `Display`, `Scheduler` |
| 07 | [Closures et itérateurs](cours/07-closures-et-iterateurs/README.md) | `Fn`, `map`/`filter`/`fold`, chaînes lazy | Filtres et stats |
| 08 | [Durées de vie et smart pointers](cours/08-lifetimes-et-smart-pointers/README.md) | `'a`, `Box`, `Rc`, `RefCell` | Vues empruntées |
| 09 | [Fichiers et sérialisation](cours/09-fichiers-et-serialisation/README.md) | `std::fs`, `io`, serde, JSON | Sauvegarde/chargement |
| 10 | [CLI et ergonomie](cours/10-cli-et-ergonomie/README.md) | clap, sous-commandes, couleurs, codes de sortie | L'interface complète |
| 11 | [Concurrence](cours/11-concurrence/README.md) | threads, `Send`/`Sync`, `Arc<Mutex>`, canaux | Import parallèle |
| 12 | [Projet final : Mémo](cours/12-projet-final-memo/README.md) | assemblage, algorithme SM-2, polish | L'application entière |

Annexes :
- [Aide-mémoire de syntaxe](cours/annexes/aide-memoire.md)
- [Décoder les erreurs du compilateur](cours/annexes/erreurs-du-compilateur.md)
- [Ressources et lectures](cours/annexes/ressources.md)

---

## Comment travailler

1. **Lis le chapitre en entier une première fois**, sans coder. Tu ne retiendras pas tout, c'est normal.
2. **Fais les exercices dans l'ordre.** Ils sont progressifs : l'exercice 3 suppose l'exercice 1 digéré.
3. **Compile souvent.** `cargo check` après chaque bloc de 5 lignes, au début. Le compilateur Rust
   est un professeur particulier : ses messages sont longs parce qu'ils contiennent la solution.
4. **Ne fuis pas devant une erreur d'emprunt.** Lis le message, relis le chapitre 2, essaie.
   Le moment où tu « comprends » l'ownership est le moment où tu deviens autonome en Rust.
5. **Écris des tests** dès le chapitre 5. Sur ce projet, ils t'éviteront de casser l'algorithme
   de révision sans t'en apercevoir.

Rythme conseillé : **un chapitre par session** de 1 h à 2 h. Le chapitre 2 mérite d'y revenir deux fois.

### Quand tu es bloqué

Dans l'ordre :
1. Relis le message d'erreur en entier (`cargo check` puis, s'il y a un code type `E0502`, `rustc --explain E0502`).
2. Cherche la méthode dans la doc locale : `cargo doc --open`, ou <https://doc.rust-lang.org/std/>.
3. Regarde la section « Indices » de l'exercice — elle te dit *quoi* chercher, pas *comment* l'écrire.
4. Demande-moi une explication de la notion. (Pas la solution : tu m'en voudras.)

---

## Organisation du dépôt

```
tuto-rust/
├── README.md              ← tu es ici
├── Cargo.toml             ← workspace : liste tes crates d'exercices
├── src/main.rs            ← bac à sable du chapitre 1
├── cours/                 ← les 13 chapitres + annexes
├── exercices/             ← un dossier (= une crate) par exercice, créé par toi
│   ├── ch01-fizzbuzz/
│   ├── ch01-temperature/
│   └── ...
└── projet/memo/           ← l'application finale, construite jalon après jalon
```

Chaque exercice est une **crate indépendante** que tu crées toi-même
(`cargo new exercices/ch01-fizzbuzz`). Le chapitre 0 explique la mécanique.

Bonne route. Commence par [le chapitre 0](cours/00-mise-en-route/README.md).
