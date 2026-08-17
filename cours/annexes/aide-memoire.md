# Aide-mémoire Rust

À garder ouvert pendant les exercices.

## Déclarations

```rust
let x = 5;                      // immuable
let mut x = 5;                  // mutable
let x: i64 = 5;                 // annoté
const MAX: u32 = 100_000;       // constante, type obligatoire
static NOM: &str = "memo";      // statique, vit tout le programme

fn f(a: i32, b: &str) -> String { }
fn g<T: Clone>(x: T) -> T { }
```

## Types

```
i8 i16 i32 i64 i128 isize        entiers signés   (défaut i32)
u8 u16 u32 u64 u128 usize        non signés       (usize = index)
f32 f64                          flottants        (défaut f64)
bool char                        char = 4 octets Unicode
(A, B)      tuple
[T; N]      tableau taille fixe
&[T]        slice
Vec<T>      tableau dynamique
String      texte possédé      &str  texte emprunté
Option<T>   Some(x) | None
Result<T,E> Ok(x)  | Err(e)
```

## Contrôle de flux

```rust
if cond { } else if cond { } else { }
let v = if cond { a } else { b };

match valeur {
    1 => "un",
    2 | 3 => "deux ou trois",
    4..=9 => "quelques",
    n if n > 100 => "beaucoup",
    _ => "autre",
}

if let Some(x) = opt { }
let Some(x) = opt else { return; };
while let Some(x) = pile.pop() { }

loop { break valeur; }
while cond { }
for x in collection { }
for i in 0..10 { }        // exclusif
for i in 0..=10 { }       // inclusif
'ext: for .. { break 'ext; }
```

## Structs et enums

```rust
#[derive(Debug, Clone, PartialEq)]
struct S { a: i32, b: String }
struct T(i32, f64);              // tuple struct
struct U;                        // unit struct

enum E {
    Vide,
    Tuple(i32, String),
    Struct { x: f64 },
}

impl S {
    fn new() -> Self { }         // fonction associée : S::new()
    fn lire(&self) { }           // méthode : s.lire()
    fn muter(&mut self) { }
    fn consommer(self) { }
}
```

## Ownership — la carte mémoire

```
let b = a;          move (sauf si Copy)
&a                  emprunt partagé      — autant qu'on veut
&mut a              emprunt exclusif     — un seul, et aucun partagé en même temps
a.clone()           copie profonde explicite
```

Un emprunt vit jusqu'à sa **dernière utilisation** (NLL), pas jusqu'à la fin du bloc.

## Option / Result

```rust
opt.is_some()  opt.unwrap_or(d)  opt.unwrap_or_else(|| d)  opt.unwrap_or_default()
opt.map(f)  opt.and_then(f)  opt.filter(p)  opt.ok_or(e)  opt.take()  opt.as_ref()

res.map(f)  res.map_err(f)  res.ok()  res.unwrap_or(d)  res?
```

`?` = « si Err/None, retourne tout de suite ». Uniquement dans une fonction qui renvoie
`Result`/`Option`.

## Collections

```rust
Vec:  push pop insert remove get len is_empty contains sort sort_by_key dedup retain extend
Map:  insert get get_mut remove contains_key entry(k).or_default() keys values
Set:  insert contains remove union intersection difference
Str:  len chars bytes split split_whitespace lines trim to_lowercase replace
      starts_with contains find parse push_str repeat format!
```

## Itérateurs

```rust
// adaptateurs (paresseux)
map filter filter_map flat_map flatten take skip take_while skip_while
enumerate zip chain rev peekable step_by inspect cloned copied scan

// consommateurs
collect sum product count last nth min max min_by_key max_by_key
fold reduce any all find position for_each partition unzip
```

```rust
let v: Vec<_>              = it.collect();
let r: Result<Vec<_>, _>   = it.collect();      // court-circuite au premier Err
let m: HashMap<_, _>       = it.collect();      // depuis un itérateur de paires
```

Trois modes : `.iter()` → `&T`, `.iter_mut()` → `&mut T`, `.into_iter()` → `T`.

## Traits usuels

```
Debug {:?}   Display {}   Clone   Copy   Default
PartialEq Eq   PartialOrd Ord   Hash
From/Into   TryFrom/TryInto   FromStr (.parse())
Iterator   IntoIterator   Error   Drop
```

```rust
trait T { fn obligatoire(&self); fn defaut(&self) { } }
impl T for S { }
fn f(x: &impl T) { }          // statique
fn f(x: &dyn T) { }           // dynamique
Vec<Box<dyn T>>               // hétérogène
```

## Modules

```rust
mod nom;                 // inclut nom.rs
pub  pub(crate)  pub(super)
use crate::a::b;  use super::c;  use std::collections::HashMap;
pub use crate::a::B;     // ré-export
```

## Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn t() { assert_eq!(a, b); assert!(c); }
    #[test] #[should_panic(expected = "...")] fn p() { }
}
```

`cargo test`, `cargo test nom`, `cargo test -- --nocapture`

## Formatage

```
{}      Display          {:?}    Debug         {:#?}   Debug joli
{nom}   capture          {:>8}   aligné droite {:<8}   gauche  {:^8}  centré
{:.2}   2 décimales      {:08.3} zéros + déc.  {:width$} largeur variable
{:x}    hexa             {:b}    binaire       {:e}    scientifique
```

## Pointeurs intelligents

```
Box<T>            tas, taille inconnue, récursion, dyn Trait
Rc<T>             propriétaires multiples (mono-thread)
RefCell<T>        mutabilité vérifiée à l'exécution
Rc<RefCell<T>>    partagé + mutable (mono-thread)
Arc<Mutex<T>>     partagé + mutable (multi-thread)
Cow<'a, T>        emprunté, cloné seulement si modifié
```

## Concurrence

```rust
thread::spawn(move || { }).join()
thread::scope(|s| { s.spawn(|| { }); })
Arc::clone(&x)
mutex.lock().unwrap()
let (tx, rx) = mpsc::channel();   // ne pas oublier drop(tx)
rayon: par_iter() par_chunks_mut()
```

## Cargo

```bash
cargo new nom / --lib     cargo check     cargo build [--release]
cargo run -p paquet -- args               cargo test [filtre]
cargo fmt                 cargo clippy -- -D warnings
cargo add crate --features f              cargo doc --open
cargo tree                cargo install --path .
rustc --explain E0502
```

## Codes d'erreur fréquents

```
E0382  usage d'une valeur déplacée
E0499  deux emprunts mutables
E0502  emprunt mutable alors qu'un partagé est vivant
E0505  déplacement d'une valeur empruntée
E0507  déplacement hors d'une référence
E0106  durée de vie manquante
E0308  types incompatibles
E0599  méthode inexistante (souvent : un trait à importer)
```
