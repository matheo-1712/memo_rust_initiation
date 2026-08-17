# Ressources

Une liste courte et triée. Mieux vaut trois bonnes sources qu'un signet de trente onglets.

## Documentation officielle

| Ressource | Quand |
|-----------|-------|
| [The Rust Book](https://doc.rust-lang.org/book/) | la référence. À relire **après** ce cours : tu y verras des choses que tu ne pouvais pas voir avant |
| [Doc de la std](https://doc.rust-lang.org/std/) | quotidiennement. Utilise la barre de recherche, elle est excellente |
| [Rust by Example](https://doc.rust-lang.org/rust-by-example/) | quand tu veux voir un exemple compilable d'une notion |
| [docs.rs](https://docs.rs) | la doc de **toutes** les crates publiées |
| `cargo doc --open` | la doc des versions exactes que tu utilises, hors ligne |
| [Rust Playground](https://play.rust-lang.org) | tester une idée en 10 secondes, partager un extrait |

Version française du Book : <https://jimskapt.github.io/rust-book-fr/> — correcte, mais parfois en
retard sur l'original. Prends l'habitude de l'anglais pour la doc technique, tu n'y couperas pas.

## Pratique

- **[Rustlings](https://github.com/rust-lang/rustlings)** — des centaines de petits exercices à
  corriger. Le meilleur complément à ce cours ; fais-en 15 minutes par jour.
- **[Exercism, piste Rust](https://exercism.org/tracks/rust)** — exercices avec relecture humaine.
- **[Advent of Code](https://adventofcode.com)** — en te fixant une contrainte (zéro `unwrap`,
  zéro `clone`, tout en itérateurs).
- **[100 Exercises to Learn Rust](https://rust-exercises.com/)** — parcours guidé et moderne.

## Aller plus loin

- **Rust for Rustaceans**, Jon Gjengset — le second livre. Traits avancés, variance, unsafe, design
  d'API. À lire quand ce cours te semblera facile.
- **[Rust Design Patterns](https://rust-unofficial.github.io/patterns/)** — idiomes et anti-patterns.
- **[The Rustonomicon](https://doc.rust-lang.org/nomicon/)** — le `unsafe`. Fascinant, rarement
  nécessaire.
- **[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)** — comment concevoir une
  bibliothèque que d'autres aimeront utiliser.
- **[Jon Gjengset sur YouTube](https://www.youtube.com/@jonhoo)** — des sessions de live coding de
  plusieurs heures, d'un niveau rare.

## Crates utiles à connaître

| Domaine | Crates |
|---------|--------|
| CLI | `clap`, `owo-colors`, `indicatif` (barres de progression), `dialoguer` |
| Erreurs | `thiserror` (bibliothèque), `anyhow` (application) |
| Sérialisation | `serde`, `serde_json`, `toml`, `csv` |
| Dates | `chrono`, `time`, `jiff` |
| Texte | `regex`, `unicode-width`, `similar` (diff) |
| Fichiers | `walkdir`, `glob`, `tempfile`, `directories`, `notify` |
| Parallélisme | `rayon`, `crossbeam`, `parking_lot` |
| Async / réseau | `tokio`, `reqwest`, `axum` |
| Base de données | `sqlx`, `rusqlite`, `sea-orm` |
| Tests | `proptest` (tests de propriétés), `insta` (snapshots), `criterion` (benchmarks) |
| TUI | `ratatui`, `crossterm` |
| Logs | `tracing`, `env_logger` |

Pour choisir : <https://lib.rs> est mieux catégorisé que crates.io, et <https://blessed.rs> donne
des recommandations argumentées par domaine.

## Communauté

- [users.rust-lang.org](https://users.rust-lang.org) — le forum officiel, très accueillant pour les
  débutants (et les questions y sont mieux traitées que sur Stack Overflow)
- [r/rust](https://reddit.com/r/rust) — actualité de l'écosystème
- [This Week in Rust](https://this-week-in-rust.org/) — la newsletter hebdo, excellente pour suivre
- Discord communautaire officiel : canal `#beginners`

## Outils

```bash
cargo install cargo-watch      # cargo watch -x check : recompile à chaque sauvegarde
cargo install cargo-expand     # voir ce que les macros génèrent réellement
cargo install cargo-edit       # cargo upgrade
cargo install cargo-audit      # vulnérabilités connues dans tes dépendances
cargo install cargo-nextest    # exécution des tests plus rapide et plus lisible
cargo install bacon            # un cargo-watch avec une belle interface
```

`cargo watch -x check` dans un terminal à côté de ton éditeur change le rythme de travail :
tu vois les erreurs apparaître à mesure que tu écris.

---

## Un mot sur les IA et l'apprentissage de Rust

Les assistants (moi compris) écrivent du Rust correct pour les tâches courantes. Deux pièges pendant
un apprentissage :

1. **Le code généré compile souvent sans que tu comprennes pourquoi.** L'ownership s'apprend en s'y
   cognant. Un `clone()` suggéré par une IA et accepté sans réfléchir est une leçon perdue.
2. **Les modèles reproduisent des API périmées** (c'est flagrant sur `rand`, `chrono`, `clap`, dont
   les versions majeures ont changé les signatures). Vérifie toujours dans `cargo doc --open`.

L'usage qui te fera progresser : demande des **explications** et des **relectures**
(« pourquoi cette erreur ? », « qu'est-ce qui n'est pas idiomatique ici ? »), pas des solutions.
C'est exactement le contrat de ce cours.
