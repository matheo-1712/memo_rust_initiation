# Chapitre 0 — Mise en route

**Objectif :** avoir une chaîne d'outils qui marche, savoir créer/compiler/lancer/tester un projet,
et comprendre ce que fait `cargo`.

Durée : 30 minutes. Aucun exercice de programmation ici, juste de la plomberie — mais fais-la
sérieusement, tout le reste en dépend.

---

## 1. La toolchain

Rust s'installe via **rustup**, qui gère les versions du compilateur. Tu as déjà :

```
rustc 1.91.0
cargo 1.91.0
```

installés dans `C:\Users\<toi>\.cargo\bin`.

### ⚠️ Ton PATH n'est pas configuré

Dans ton terminal, `cargo --version` répond « n'est pas reconnu ». Les binaires existent, mais
Windows ne sait pas où les chercher. Corrige-le une fois pour toutes :

```bash
setx PATH "$env:PATH;$env:USERPROFILE\.cargo\bin"
```

Ferme et rouvre ton terminal, puis vérifie :

```bash
cargo --version
```

Si ça ne marche toujours pas : `Paramètres Windows → Variables d'environnement → Path (utilisateur)
→ Nouveau → %USERPROFILE%\.cargo\bin`.

### Les composants utiles

```bash
rustup component add rustfmt clippy
```

- **rustfmt** formate ton code (`cargo fmt`). Ne discute pas avec lui, il a toujours raison.
- **clippy** est un linter (`cargo clippy`). Il t'apprendra énormément d'idiomes Rust :
  traite ses avertissements comme un cours particulier gratuit.

### L'éditeur

Tu as un dossier `.idea/` : tu utilises **RustRover** (JetBrains), c'est parfait, tout est intégré.
Si tu passes sur VS Code, installe l'extension **rust-analyzer** (celle de « The Rust Programming
Language », pas l'ancienne « Rust »).

Le vrai gain d'un bon éditeur en Rust : il t'affiche les **types inférés** en ligne. Utilise-le
constamment pour comprendre ce que le compilateur déduit.

---

## 2. Cargo, le couteau suisse

Cargo est à la fois le gestionnaire de paquets, le système de build, le lanceur de tests et le
générateur de doc. Les commandes que tu vas taper mille fois :

| Commande | Ce qu'elle fait |
|----------|-----------------|
| `cargo new mon-projet` | crée une crate binaire (avec `src/main.rs`) |
| `cargo new --lib ma-lib` | crée une crate bibliothèque (avec `src/lib.rs`) |
| `cargo check` | **compile sans produire de binaire** — ultra rapide, ton réflexe n°1 |
| `cargo build` | compile en mode debug → `target/debug/` |
| `cargo build --release` | compile optimisé (10 à 100× plus rapide à l'exécution, plus lent à compiler) |
| `cargo run` | compile puis exécute |
| `cargo run -- arg1 arg2` | idem, en passant des arguments à **ton** programme (le `--` sépare) |
| `cargo test` | lance les tests |
| `cargo fmt` | formate |
| `cargo clippy` | lint |
| `cargo add serde` | ajoute une dépendance au `Cargo.toml` |
| `cargo doc --open` | génère et ouvre la doc de ton projet **et de toutes tes dépendances** |

`cargo doc --open` est sous-estimé : c'est la doc **exacte** des versions que tu utilises,
hors ligne. Prends l'habitude.

---

## 3. Anatomie d'une crate

Une **crate** est l'unité de compilation. Un **paquet** (package) contient une ou plusieurs crates
et se décrit dans un `Cargo.toml` :

```toml
[package]
name = "tuto-rust"
version = "0.1.0"
edition = "2024"

[dependencies]
```

- `edition` = le « millésime » du langage. La 2024 est la plus récente ; elle change quelques
  règles de syntaxe mais **pas** la bibliothèque standard. Tout ce qu'on écrira sera en édition 2024.
- `[dependencies]` : les crates externes, tirées de <https://crates.io>.
- `Cargo.lock` : les versions exactes résolues. On le commite pour un binaire (c'est ton cas).

Le point d'entrée d'un binaire est toujours :

```rust
fn main() {
    println!("Hello, world!");
}
```

`println!` avec un `!` est une **macro**, pas une fonction. On y reviendra ; retiens juste que
le `!` signale une macro, et que les macros peuvent faire des choses qu'une fonction ne peut pas
(ici : vérifier ton format à la compilation).

---

## 4. Le workspace de ce cours

Tu vas créer des dizaines de petites crates d'exercice. Sans précaution, chacune aurait son propre
dossier `target/` (plusieurs centaines de Mo au total) et il faudrait `cd` partout.

La solution : un **workspace**. J'ai préparé le `Cargo.toml` racine pour toi :

```toml
[workspace]
members = ["exercices/*", "projet/memo"]
```

Conséquences :
- toutes les crates listées partagent **un seul** `target/` à la racine ;
- depuis la racine, `cargo run -p ch01-fizzbuzz` lance n'importe quel membre (`-p` = package) ;
- `cargo test` à la racine teste **tout** le workspace.

### Créer un exercice

Depuis la racine du dépôt :

```bash
cargo new exercices/ch01-fizzbuzz
```

Cargo détecte le workspace et ajoute la crate automatiquement (le glob `exercices/*` la capture).
Ensuite :

```bash
cargo run -p ch01-fizzbuzz
```

> Convention de nommage pour ce cours : `chXX-nom-de-lexercice`, en minuscules avec des tirets.

---

## 5. Lire une erreur de compilation

Rust n'est pas un langage où l'on code puis où l'on compile pour voir. C'est un langage où l'on
**dialogue avec le compilateur**. Un message type :

```
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:4:20
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let t = s;
  |             - value moved here
4 |     println!("{}", s);
  |                    ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
```

Sa structure, toujours la même :
1. `error[EXXXX]` : un **code**. `rustc --explain E0382` t'en donne une page complète avec exemples.
2. La ligne fautive, fléchée.
3. Des annotations qui racontent **l'histoire** de la valeur (« déplacée ici », « empruntée là »).
4. Souvent un `help:` avec la correction exacte.

Le réflexe à prendre : **lis de bas en haut**. Le `help:` en dernier ligne est souvent la réponse.

Et surtout : une erreur de compilation Rust n'est pas un échec, c'est un bug que tu n'auras pas
en production. C'est le contrat du langage.

---

## ✅ Checklist avant le chapitre 1

- [ ] `cargo --version` répond dans un terminal neuf
- [ ] `rustup component add rustfmt clippy` est passé
- [ ] `cargo run` à la racine affiche `Hello, world!`
- [ ] `cargo new exercices/ch00-test` fonctionne, puis `cargo run -p ch00-test` affiche `Hello, world!`
- [ ] tu as ouvert `cargo doc --open` une fois pour voir à quoi ça ressemble
- [ ] tu sais ce que fait `rustc --explain E0382`

Supprime `exercices/ch00-test` quand tu as vérifié, et passe au
[chapitre 1](../01-bases-du-langage/README.md).
