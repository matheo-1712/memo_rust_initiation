# Chapitre 5 — Modules, tests et crates

**Objectif :** sortir du fichier unique. Organiser le code, contrôler ce qui est public, écrire des
tests qui te protègent, et utiliser des bibliothèques externes.

Mémo commence à grossir : ce chapitre arrive au bon moment.

---

## 1. Modules

Un module est un espace de noms **et** une frontière de visibilité.

```rust
mod carte {                     // module inline
    pub struct Card { ... }     // public HORS du module
    fn interne() { }            // privé : invisible depuis l'extérieur

    pub mod revision {          // sous-module
        pub fn planifier() { }
    }
}

carte::revision::planifier();
```

**Tout est privé par défaut.** `pub` ouvre. Un module enfant voit les items privés de ses ancêtres ;
l'inverse est faux.

Niveaux de visibilité utiles :

```rust
pub          // partout
pub(crate)   // partout dans MA crate, invisible pour les utilisateurs de ma bibliothèque
pub(super)   // seulement pour le module parent
```

`pub(crate)` est le bon défaut quand tu découpes une application en modules.

### Modules dans des fichiers

La forme moderne (édition 2018+), celle à utiliser :

```
src/
├── main.rs          → déclare `mod card;` `mod library;` `mod scheduler;`
├── card.rs          → module `card`
├── library.rs       → module `library`
└── scheduler/
    ├── mod.rs       ← ancienne convention, ok mais démodée
    └── sm2.rs
```

ou, préféré aujourd'hui :

```
src/
├── main.rs
├── scheduler.rs     → contient `pub mod sm2;`
└── scheduler/
    └── sm2.rs
```

**Le point qui coince toujours au début :** `mod foo;` ne veut pas dire « importe » mais
**« inclus le fichier foo.rs ici, comme module »**. Tu ne le déclares donc **qu'une fois**, dans la
racine (`main.rs`/`lib.rs`) ou dans son module parent. Depuis les autres fichiers, tu ne
redéclares pas : tu utilises `use crate::foo::...`.

### `use`

```rust
use std::collections::HashMap;
use crate::card::{Card, CardState};      // chemin absolu depuis la racine de la crate
use super::helpers::normaliser;          // relatif au parent
use crate::error::MemoError as Erreur;   // alias
pub use crate::card::Card;               // ré-export : rend Card accessible depuis ce module
```

`pub use` sert à composer une API propre : tes utilisateurs écrivent `memo::Card` sans savoir que
c'est défini dans `memo::card::Card`.

### Binaire + bibliothèque dans le même paquet

Le motif idéal pour une application testable :

```
src/
├── lib.rs      ← toute la logique, testable, réutilisable  (crate `memo`)
└── main.rs     ← seulement l'interface CLI, appelle `memo::...`
```

Dans `main.rs`, tu utilises alors `use memo::Library;` (le nom du paquet), pas `crate::`.
C'est exactement ce que tu feras pour Mémo au chapitre 10.

---

## 2. Tests

### Tests unitaires

Ils vivent **dans** le fichier qu'ils testent, en bas :

```rust
#[cfg(test)]                       // compilé uniquement pour `cargo test`
mod tests {
    use super::*;                  // importe tout le module parent, y compris le privé

    #[test]
    fn ajoute_une_carte() {
        let mut lib = Library::new();
        let id = lib.add("rust", "Q", "R").unwrap();
        assert_eq!(lib.get(id).unwrap().question, "Q");
    }

    #[test]
    fn refuse_une_question_vide() {
        let mut lib = Library::new();
        assert!(lib.add("rust", "", "R").is_err());
    }

    #[test]
    #[should_panic(expected = "index")]
    fn panique_bien() { }

    #[test]
    #[ignore = "lent"]
    fn test_lourd() { }            // lancé avec `cargo test -- --ignored`
}
```

Les macros d'assertion :

```rust
assert!(condition, "message avec {valeur}");
assert_eq!(a, b);
assert_ne!(a, b);
```

Un test qui renvoie `Result` peut utiliser `?` :

```rust
#[test]
fn charge_le_fichier() -> Result<(), Box<dyn std::error::Error>> {
    let lib = Library::load("fixtures/exemple.json")?;
    assert_eq!(lib.len(), 3);
    Ok(())
}
```

### Tests d'intégration

Dans `tests/` à la racine du paquet. Chaque fichier est une crate séparée qui ne voit que l'**API
publique** — c'est exactement le point de vue de tes futurs utilisateurs.

```
projet/memo/
├── src/lib.rs
└── tests/
    └── revision.rs      → `use memo::*;`
```

### Lancer les tests

```bash
cargo test                      # tout
cargo test -p memo              # un paquet du workspace
cargo test ajoute               # les tests dont le nom contient "ajoute"
cargo test -- --nocapture       # affiche les println! des tests
cargo test -- --test-threads=1  # séquentiel (utile si tests d'IO sur le même fichier)
```

### Que tester ?

Pas « tout », mais :
- les **règles métier** (un intervalle ne doit jamais diminuer après une bonne réponse) ;
- les **cas limites** (liste vide, un seul élément, valeur maximale, chaîne UTF-8 multi-octets) ;
- les **bugs corrigés** (un test par bug : il ne reviendra pas) ;
- ce qui est **difficile à vérifier à la main** (l'algorithme SM-2 du chapitre 12).

Pas besoin de tester les accesseurs triviaux. Vise l'utilité, pas le pourcentage.

---

## 3. Documentation

```rust
/// Ajoute une carte au paquet et renvoie son identifiant.
///
/// # Erreurs
/// Renvoie [`MemoError::QuestionVide`] si la question est vide.
///
/// # Exemples
/// ```
/// let mut lib = memo::Library::new();
/// let id = lib.add("rust", "Q ?", "R").unwrap();
/// assert_eq!(id, 1);
/// ```
pub fn add(&mut self, ...) -> Result<u32, MemoError> { }
```

Détail remarquable : **les exemples de doc sont exécutés par `cargo test`**. Ta documentation ne peut
donc pas mentir sur la compilation. Peu de langages offrent ça ; utilise-le.

`//!` documente le **conteneur** (à mettre en haut d'un fichier de module) au lieu de l'item suivant.

`cargo doc --open` génère le site HTML.

---

## 4. Dépendances externes

```bash
cargo add serde --features derive -p memo
cargo add serde_json chrono clap rand
cargo add --dev tempfile           # dépendance de test uniquement
```

Dans `Cargo.toml` :

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[dev-dependencies]
tempfile = "3"
```

Les **features** sont des options de compilation d'une crate. Beaucoup de crates ont un cœur minimal
et des features à activer (`serde/derive`, `chrono/serde`, `clap/derive`). Lis toujours la section
« Feature flags » de la doc d'une crate, c'est là que se cachent les fonctions qui « n'existent pas ».

Le versionnage `"1"` signifie « ≥ 1.0.0 et < 2.0.0 » (sémantique compatible). `Cargo.lock` fige la
version exacte réellement utilisée.

Où chercher une crate : <https://crates.io> (popularité), <https://lib.rs> (mieux catégorisé),
et la doc sur <https://docs.rs>. Critères de choix : téléchargements récents, dernière publication,
nombre de dépendances tirées.

---

## Exercices

### 5.1 — `ch05-geometrie`

Reprends ton exercice 3.1 (`ch03-formes`) et restructure-le :

```
src/
├── lib.rs          → `pub mod forme; pub mod stats;` + `pub use forme::Forme;`
├── forme.rs        → l'enum + ses méthodes
├── stats.rs        → aire totale, forme max, moyenne des périmètres
└── main.rs         → uniquement l'affichage
tests/
└── api.rs          → tests d'intégration via l'API publique
```

**Contraintes :**
- au moins une fonction `pub(crate)` utilisée entre modules mais absente de l'API publique ;
- 6 tests unitaires + 2 tests d'intégration ;
- un exemple de doc exécutable sur `Forme::aire` ;
- `cargo doc --open` doit produire une page lisible.

**Réussi si :** `cargo test` affiche des résultats dans les trois catégories (unit tests, integration
tests, doc-tests).

---

### 5.2 — `ch05-tdd-roman`

Exercice de **test-first** : tu écris les tests d'abord, l'implémentation ensuite.

Fonction cible : `fn vers_romain(n: u32) -> Result<String, String>` et
`fn depuis_romain(s: &str) -> Result<u32, String>` (1 à 3999).

**Marche à suivre imposée :**
1. Écris 12 tests (`1 → I`, `4 → IV`, `9 → IX`, `14`, `40`, `90`, `400`, `1994 → MCMXCIV`, `3999`,
   `0 → Err`, `4000 → Err`, aller-retour aléatoire).
2. Lance `cargo test` : les 12 échouent (rouge).
3. Implémente jusqu'au vert, sans toucher aux tests.
4. Refactorise en gardant le vert.

**Indice de conception :** un tableau de paires `(valeur, symbole)` en ordre décroissant rend
`vers_romain` très court. Pour l'aller-retour, `depuis_romain(&vers_romain(n)?)? == n` pour tout n.

---

### 5.3 — `ch05-memo-modules` 🎯 **JALON MÉMO 3/9**

Découpe `projet/memo` et blinde-le de tests.

**Structure attendue :**

```
projet/memo/
├── Cargo.toml
├── src/
│   ├── lib.rs          → `pub mod card; pub mod library; pub mod error;` + ré-exports
│   ├── card.rs         → Card, CardState, Grade
│   ├── library.rs      → Library
│   ├── error.rs        → MemoError
│   └── main.rs         → `use memo::*;` — juste une démo pour l'instant
└── tests/
    └── library.rs
```

**Contraintes :**
- `main.rs` ne doit contenir **aucune logique métier** : uniquement de l'affichage et des appels ;
- `lib.rs` ré-exporte `Card`, `CardState`, `Grade`, `Library`, `MemoError` pour qu'on écrive
  `memo::Card` et pas `memo::card::Card` ;
- les champs de `Card` deviennent **privés**, avec des accesseurs (`fn question(&self) -> &str`).
  Tu vas devoir corriger tous les appels : c'est l'exercice.
- au moins **10 tests unitaires** couvrant : ajout, doublon refusé, question vide refusée,
  suppression d'un id inexistant, `due_cards` avec et sans filtre de paquet, `count_by_deck`,
  suspension, et un cas UTF-8 (`preview` sur une question pleine d'accents et d'emojis) ;
- 2 tests d'intégration dans `tests/library.rs` ;
- de la doc `///` sur tous les items publics, avec **au moins un exemple exécutable**.

**Vérification :**

```bash
cargo test -p memo
cargo clippy -p memo -- -D warnings
cargo fmt --check
```

Les trois doivent passer. Clippy va probablement râler sur `new()` sans `Default`, sur des
`&Vec<T>` au lieu de `&[T]`, sur des `return` inutiles : **corrige chaque avertissement** et lis
son explication. C'est ton meilleur professeur d'idiomes cette semaine.

---

### 5.4 — `ch05-choisir-une-crate` (recherche)

Sans écrire beaucoup de code : choisis une crate pour générer un identifiant unique
(`uuid`, `nanoid`, `ulid`…), ajoute-la, et écris un petit programme qui en imprime 5.

Puis rédige dans le `README.md` de la crate d'exercice : pourquoi celle-ci, quelle version,
combien de dépendances elle tire (`cargo tree | measure` ou `cargo tree -e normal`), et ce que tu
aurais choisi si la contrainte avait été « zéro dépendance ».

**But :** apprendre à *évaluer* une dépendance. Dans l'écosystème Rust, la facilité de `cargo add`
est un piège : chaque crate ajoutée est du code que tu adoptes.

---

## ✅ Auto-évaluation

1. Que fait exactement `mod foo;` ?
2. Différence entre `pub` et `pub(crate)` ?
3. Pourquoi mettre la logique dans `lib.rs` plutôt que dans `main.rs` ?
4. Où vivent les tests unitaires ? Les tests d'intégration ? Qu'est-ce qu'ils voient chacun ?
5. À quoi sert `#[cfg(test)]` ?
6. Qu'est-ce qu'une feature de crate ?

→ [Chapitre 6 : Traits et génériques](../06-traits-et-generiques/README.md)
