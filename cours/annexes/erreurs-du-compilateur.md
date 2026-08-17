# Décoder les erreurs du compilateur

Le compilateur Rust est le meilleur professeur de ce cours. Cette page traduit les messages que tu
vas croiser, du plus fréquent au plus obscur.

Réflexes de base :
1. lis **de bas en haut** : le `help:` final contient souvent la correction exacte ;
2. `rustc --explain E0502` pour la page complète avec exemples ;
3. **une erreur à la fois** : la première cause souvent les suivantes.

---

## Ownership et emprunts

### `E0382: use of moved value`

```
let s = String::from("x");
let t = s;
println!("{s}");   // ← ici
```

Tu utilises une valeur qui a été déplacée (affectation, passage à une fonction, `for x in v`,
capture par `move`).

**Corrections, de la meilleure à la pire :**
- emprunte au lieu de déplacer : `let t = &s;` ou `f(&s)` ;
- si c'est une boucle : `for x in &v` au lieu de `for x in v` ;
- `s.clone()` si tu as vraiment besoin de deux valeurs indépendantes ;
- rends la valeur depuis la fonction (`-> String`).

### `E0502: cannot borrow as mutable because also borrowed as immutable`

```
let premier = &v[0];
v.push(4);          // ← ici
println!("{premier}");
```

Un lecteur est encore vivant quand tu veux écrire.

**Corrections :** copie la valeur lue (`let premier = v[0];`) ; ou déplace l'usage du lecteur
**avant** la mutation (l'emprunt s'arrête à sa dernière utilisation) ; ou sépare en deux étapes
avec un bloc `{ }`.

### `E0499: cannot borrow as mutable more than once`

Deux `&mut` simultanés sur la même valeur. Typiquement `f(&mut v, &mut v[0])`, ou une méthode `&mut
self` appelée avec un argument emprunté à `self`.

**Corrections :** extrais la valeur nécessaire dans une variable locale avant l'appel ; ou utilise
`split_at_mut` / `iter_mut` pour obtenir des `&mut` disjoints que le compilateur sait distinguer.

### `E0505: cannot move out of ... because it is borrowed`

Tu déplaces une valeur alors qu'une référence vers elle existe encore. Même remède : réduis la
portée de l'emprunt.

### `E0507: cannot move out of borrowed content`

Tu essaies de sortir une valeur d'une référence (souvent `*ref` ou un champ d'un `&self`).

**Corrections :** `.clone()`, `.to_owned()` ; ou travaille avec la référence (renvoie `&T` au lieu
de `T`) ; ou `std::mem::take(&mut champ)` si tu peux laisser une valeur par défaut à la place ;
ou `Option::take()` pour un champ `Option`.

### `E0596: cannot borrow as mutable`

Il manque un `mut` sur la déclaration (`let mut v`) ou sur le paramètre (`&mut T`).

---

## Types

### `E0308: mismatched types`

Le plus fréquent, et le plus facile. Regarde `expected` / `found`. Causes classiques :

- **un `;` de trop** en fin de fonction → `expected i32, found ()` ;
- pas de conversion implicite : `as`, `i64::from(x)`, `x.into()` ;
- `&String` vs `&str` → un `&` en trop ou en moins (le déréférencement automatique fait beaucoup,
  mais pas tout) ;
- tu as oublié de sortir de l'`Option`/`Result` (`expected i32, found Option<i32>`).

### `E0277: the trait bound ... is not satisfied`

Le type ne sait pas faire ce que tu demandes.

- `X doesn't implement Debug` → ajoute `#[derive(Debug)]` sur le type ;
- `X doesn't implement Display` → utilise `{:?}` au lieu de `{}`, ou implémente `Display` ;
- `the ? operator can only be used in a function that returns Result` → change la signature ;
- `?` dans une fonction dont le type d'erreur ne correspond pas → il manque un `impl From<A> for B`.

### `E0599: no method named ... found`

Trois causes, dans cet ordre de probabilité :
1. **il manque un `use`** : les méthodes de trait exigent que le trait soit importé
   (`use std::io::Write;` pour `flush`, `use std::fmt::Write;` pour `write!` sur une `String`) ;
2. faute de frappe, ou méthode qui n'existe que sur un type voisin (`Vec` vs slice vs itérateur) ;
3. la borne générique manquante : ta fonction `<T>` n'a pas déclaré `T: Clone`, donc `.clone()`
   n'existe pas.

### `E0433: failed to resolve` / `cannot find ... in this scope`

Chemin de module faux. Rappels : `crate::` = racine de ta crate, `super::` = parent,
`nom_du_paquet::` depuis `main.rs` vers `lib.rs`. Et vérifie que `mod x;` est bien déclaré **une
seule fois**, dans le bon fichier parent.

---

## Durées de vie

### `E0106: missing lifetime specifier`

Tu renvoies une référence sans que le compilateur puisse savoir de quelle entrée elle vient.

**Corrections :** annote (`fn f<'a>(a: &'a str, b: &str) -> &'a str`) ; ou renvoie une valeur
possédée (`String`) — c'est souvent le bon choix, ne t'acharne pas sur le zéro-copie ;
ou ajoute un paramètre de durée de vie à ta struct (`struct S<'a> { x: &'a str }`).

### `E0597: ... does not live long enough`

Une référence survit à la valeur pointée. Presque toujours : une valeur créée dans un bloc `{}` et
référencée à l'extérieur, ou une temporaire (`&format!(...)` stocké quelque part).

**Correction :** fais vivre la valeur plus longtemps en la déclarant dans la portée supérieure,
ou possède-la.

### `E0515: cannot return reference to local variable`

Tu renvoies une référence vers une variable de la fonction, qui va être détruite. Renvoie la valeur
elle-même. En C, ce bug compile et corrompt la pile ; ici, il est impossible.

---

## Traits et généricité

### `the trait ... cannot be made into an object`

Tu veux un `dyn Trait` mais le trait n'est pas *object-safe* : il a une méthode générique, ou qui
renvoie `Self`, ou une fonction associée sans `self`.

**Corrections :** enlève ces méthodes du trait (mets-les dans un trait annexe), ou utilise un
générique `<T: Trait>` au lieu de `dyn`.

### `conflicting implementations` / règle d'orphelin

Tu implémentes un trait externe pour un type externe. Emballe le type dans un newtype
(`struct MonVec(Vec<T>)`).

### `E0507` sur un `HashMap`

`map.get(k)` renvoie `Option<&V>`, pas `Option<V>`. Utilise `.cloned()`, `.copied()`, ou travaille
avec la référence. `map.remove(k)` te donne la valeur possédée si tu veux la sortir.

---

## Concurrence

### `` `Rc<T>` cannot be sent between threads safely ``

Remplace `Rc` par `Arc`.

### `` `RefCell<T>` cannot be shared between threads safely ``

Remplace `RefCell` par `Mutex` ou `RwLock`.

### `closure may outlive the current function`

Ajoute `move` devant la closure du thread.

### `borrowed data escapes outside of function`

Souvent : tu veux passer une référence locale à un `thread::spawn`. Utilise `thread::scope`, ou
`Arc`, ou déplace la donnée.

---

## Avertissements de clippy à connaître

Ce ne sont pas des erreurs, mais chacun t'apprend un idiome :

| Lint | Ce qu'il t'apprend |
|------|--------------------|
| `needless_range_loop` | `for x in &v` plutôt que `for i in 0..v.len()` |
| `ptr_arg` | prends `&[T]` / `&str` en paramètre, pas `&Vec<T>` / `&String` |
| `new_without_default` | si tu as `new()` sans argument, implémente `Default` |
| `redundant_clone` | ce `clone()` ne sert à rien |
| `manual_map`, `option_if_let_else` | `opt.map(...)` plutôt qu'un `match` |
| `collapsible_if` | fusionne les `if` imbriqués |
| `len_zero` | `v.is_empty()` plutôt que `v.len() == 0` |
| `single_char_pattern` | `split('x')` plutôt que `split("x")` |
| `unwrap_used` (option stricte) | traite l'erreur au lieu de l'ignorer |

Active-le en mode strict de temps en temps :

```bash
cargo clippy --all-targets -- -D warnings -W clippy::pedantic
```

Le mode `pedantic` est bavard et parfois excessif — mais lis chaque suggestion une fois, c'est un
condensé de culture Rust.
