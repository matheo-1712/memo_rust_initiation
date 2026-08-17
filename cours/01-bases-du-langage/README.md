# Chapitre 1 — Les bases du langage

**Objectif :** écrire des programmes qui calculent, décident et bouclent. Comprendre le système de
types de base et pourquoi Rust t'oblige à être explicite.

---

## 1. Variables : immuables par défaut

```rust
let x = 5;          // immuable
let mut y = 5;      // mutable
y += 1;             // ok
// x += 1;          // ERREUR : cannot assign twice to immutable variable
```

C'est **le** renversement mental par rapport aux autres langages : en Rust, la mutabilité est
une exception qu'on déclare. Ça paraît pénible pendant deux jours, puis ça devient un filet de
sécurité : quand tu vois `let mut`, tu sais que la valeur va changer.

### Shadowing

Tu peux redéclarer un nom déjà pris :

```rust
let saisie = "42";                       // &str
let saisie: i32 = saisie.trim().parse().unwrap();  // i32, même nom
```

Ce n'est pas de la mutation : c'est une **nouvelle variable** qui masque l'ancienne. C'est idiomatique
pour transformer une valeur (texte → nombre) sans inventer `saisie_str`, `saisie_int`, etc.

### Constantes

```rust
const MAX_CARTES: usize = 10_000;   // type obligatoire, calculée à la compilation, SCREAMING_CASE
```

---

## 2. Les types scalaires

| Famille | Types | Notes |
|---------|-------|-------|
| Entiers signés | `i8` `i16` `i32` `i64` `i128` `isize` | défaut : **`i32`** |
| Entiers non signés | `u8` `u16` `u32` `u64` `u128` `usize` | `usize` = taille d'un index mémoire |
| Flottants | `f32` `f64` | défaut : **`f64`** |
| Booléen | `bool` | `true` / `false`, jamais 0/1 |
| Caractère | `char` | **4 octets**, un scalaire Unicode : `'é'`, `'漢'`, `'🦀'` |

Points importants :

- **Pas de conversion implicite.** `let a: i64 = mon_i32;` échoue. Il faut `mon_i32 as i64`,
  ou mieux `i64::from(mon_i32)`. C'est verbeux et volontaire : les conversions silencieuses sont
  une source de bugs classiques.
- **`usize` pour les index et les tailles.** `vec.len()` renvoie un `usize`. Tu vas beaucoup jongler.
- **Le débordement panique en debug** et boucle en release. Pour être explicite, utilise
  `checked_add` (→ `Option`), `saturating_sub`, `wrapping_mul`.
- Séparateur de lisibilité : `1_000_000`.

### Types composés

```rust
let point: (i32, i32) = (3, 4);
let (x, y) = point;              // déstructuration
let premier = point.0;           // accès par index

let notes: [u32; 5] = [10, 12, 14, 16, 18];   // tableau de TAILLE FIXE
let zeros = [0u8; 32];                         // 32 zéros
println!("{}", notes.len());                   // 5
```

Un `[T; N]` a sa taille dans son type. Pour une liste qui grandit, c'est `Vec<T>` (chapitre 4).

---

## 3. Fonctions

```rust
fn aire(largeur: f64, hauteur: f64) -> f64 {
    largeur * hauteur       // pas de `return`, pas de `;` : c'est l'EXPRESSION finale
}

fn saluer(nom: &str) {      // pas de `-> ...` : renvoie `()`, le "unit type"
    println!("Bonjour {nom} !");
}
```

Le typage des paramètres et du retour est **obligatoire** : Rust n'infère jamais la signature
d'une fonction, seulement l'intérieur des corps.

### Expression vs instruction — la notion à ne pas rater

Presque tout est une **expression** en Rust (produit une valeur) :

```rust
let categorie = if age >= 18 { "majeur" } else { "mineur" };

let carre = {
    let n = 4;
    n * n            // pas de `;` → la valeur du bloc
};

let etat = match code {
    0 => "ok",
    1..=9 => "avertissement",
    _ => "erreur",
};
```

La règle : **un `;` transforme une expression en instruction** et jette sa valeur. C'est la cause
n°1 de l'erreur « expected `i32`, found `()` » chez les débutants : tu as mis un point-virgule
à la dernière ligne de ta fonction.

---

## 4. Contrôle de flux

```rust
if n % 2 == 0 { ... } else if n < 0 { ... } else { ... }
```

La condition doit être un `bool`. `if 1 { }` ne compile pas — pas de « valeur truthy » en Rust.

```rust
loop { ... break; }                 // boucle infinie
let v = loop { break 42; };         // `break` peut renvoyer une valeur !

while restant > 0 { ... }

for note in notes {           }     // itère sur les éléments
for i in 0..5 { }                   // 0,1,2,3,4  (exclusif)
for i in 0..=5 { }                  // 0..5 inclus
for (i, n) in notes.iter().enumerate() { }   // index + valeur
```

`for` est la boucle par défaut en Rust : elle ne peut pas sortir des bornes, contrairement à un
`while i < len` où tu peux te tromper d'inégalité.

Étiquettes pour les boucles imbriquées :

```rust
'externe: for i in 0..10 {
    for j in 0..10 {
        if i * j > 50 { break 'externe; }
    }
}
```

---

## 5. Afficher des choses

```rust
println!("Bonjour {}", nom);        // positionnel
println!("Bonjour {nom}");          // capture de variable (préféré)
println!("{:?}", monTuple);         // format DEBUG — pour les types qui dérivent Debug
println!("{:#?}", maStruct);        // debug "joli", multiligne
println!("{:>8.2}", 3.14159);       // aligné à droite sur 8 colonnes, 2 décimales → "    3.14"
println!("{:03}", 7);               // "007"
eprintln!("attention");             // sur stderr
```

`{}` exige que le type implémente `Display` (fait pour toi seulement pour les types de base) ;
`{:?}` exige `Debug` (que tu obtiens gratuitement avec `#[derive(Debug)]`, chapitre 3).

---

## 6. Lire l'entrée standard

Pour les exercices interactifs, le minimum vital :

```rust
use std::io::{self, Write};

let mut ligne = String::new();
io::stdin().read_line(&mut ligne).expect("lecture impossible");
let n: i32 = ligne.trim().parse().expect("pas un nombre");
```

Trois choses à retenir :
- `read_line` **ajoute** à la string existante et garde le `\n` → d'où le `.trim()` systématique ;
- `parse()` a besoin de savoir vers quoi parser : soit par annotation (`let n: i32`),
  soit par turbofish `ligne.trim().parse::<i32>()` ;
- `.expect("msg")` fait planter le programme proprement si ça échoue. C'est acceptable **pour
  l'instant** ; au chapitre 4 tu apprendras à traiter l'erreur au lieu de la subir.

Pour afficher une invite sans retour à la ligne, il faut vider le tampon :

```rust
print!("Ton choix > ");
io::stdout().flush().unwrap();
```

---

## Les pièges du chapitre

| Symptôme | Cause |
|----------|-------|
| `expected i32, found ()` | un `;` de trop en fin de fonction |
| `cannot assign twice to immutable variable` | il manque `mut` |
| `mismatched types: expected u32, found i32` | pas de conversion implicite : `as`, ou fixe le type au départ |
| `cannot find value x in this scope` | une variable meurt à la fin de son bloc `{}` |
| `attempt to subtract with overflow` | tu as soustrait sous zéro sur un type non signé (`usize`) |

---

## Exercices

Crée chaque exercice avec `cargo new exercices/<nom>` depuis la racine, puis
`cargo run -p <nom>`.

### 1.1 — `ch01-fizzbuzz` (échauffement)

Affiche les nombres de 1 à 100, un par ligne. Remplace les multiples de 3 par `Fizz`, de 5 par
`Buzz`, de 15 par `FizzBuzz`.

**Contrainte :** écris une fonction `fn fizzbuzz(n: u32) -> String` et fais boucler `main` dessus.
Le `match` sur un tuple `(n % 3, n % 5)` est plus élégant qu'une cascade de `if` — essaie les deux.

**Réussi si :** la ligne 15 dit `FizzBuzz`, la ligne 98 dit `98`.

---

### 1.2 — `ch01-temperature`

Un convertisseur interactif. Le programme demande une température et une unité, puis affiche la
conversion dans l'autre unité, arrondie à une décimale.

```
Température > 37.5
Unité (C/F) > C
37.5 °C = 99.5 °F
```

**Contraintes :**
- deux fonctions : `fn c_vers_f(c: f64) -> f64` et `fn f_vers_c(f: f64) -> f64` ;
- accepte `C`, `c`, `F`, `f` (regarde `to_uppercase` ou `eq_ignore_ascii_case`) ;
- si l'unité est inconnue, affiche un message d'erreur sur **stderr** et rien d'autre.

**Indices :** formules `F = C × 9/5 + 32` et `C = (F − 32) × 5/9`. Attention : `9/5` en entiers vaut
`1`. Utilise `9.0 / 5.0`. Pour l'arrondi d'affichage, `{:.1}`.

---

### 1.3 — `ch01-statistiques`

Sur un tableau de notes codé en dur (`[u32; 10]`), calcule et affiche :
- la moyenne (en `f64`, 2 décimales) ;
- le minimum et le maximum ;
- l'étendue (max − min) ;
- le nombre de notes ≥ 10.

**Contraintes :** une fonction par statistique, chacune prenant `&[u32]` en paramètre (une *slice* :
tu verras au chapitre 2 pourquoi c'est mieux que `[u32; 10]`). Interdiction d'utiliser `.iter().sum()`
ou `.max()` pour cette fois — écris les boucles à la main, c'est l'objet de l'exercice. Tu les
réécriras en une ligne au chapitre 7, et tu mesureras le chemin parcouru.

**Piège :** la moyenne d'entiers. `somme / n` en `u32` tronque. Convertis **avant** de diviser.

---

### 1.4 — `ch01-devine`

Le classique : le programme tire un nombre entre 1 et 100, l'utilisateur devine, le programme
répond « trop grand » / « trop petit » jusqu'à la victoire, puis affiche le nombre de tentatives.

**Contraintes :**
- ajoute la dépendance : `cargo add rand -p ch01-devine` ;
- ⚠️ l'API de `rand` a changé récemment : **ne recopie pas un tutoriel de 2021**. Ouvre la vraie doc
  de la version installée avec `cargo doc -p rand --open` et cherche comment obtenir un entier dans
  un intervalle. (C'est un exercice de lecture de doc autant que de code.)
- une saisie non numérique ne doit **pas** faire planter le programme : redemande. Indice :
  `match ligne.trim().parse() { Ok(n) => ..., Err(_) => continue }`.

**Bonus :** limite à 7 essais, et affiche `Perdu ! C'était 63.` à l'épuisement.

---

### 1.5 — `ch01-pyramide` (mise en forme)

Demande une hauteur `n` (1 à 20) et dessine une pyramide d'étoiles centrée :

```
Hauteur > 4
   *
  ***
 *****
*******
```

**Contraintes :** utilise le formatage de `println!` plutôt que des concaténations manuelles.
Cherche `{:width$}` et la répétition de chaînes (`"*".repeat(n)`). Rejette une hauteur hors bornes.

**Bonus :** dessine-la creuse (uniquement les bords).

---

## ✅ Auto-évaluation

Tu peux passer au chapitre 2 si tu sais répondre sans regarder :

1. Quelle est la différence entre `let x = 5;` et `let mut x = 5;` ?
2. Pourquoi `fn f() -> i32 { 5; }` ne compile pas ?
3. Que vaut le type de `notes.len()` ?
4. Pourquoi `.trim()` après un `read_line` ?
5. Quelle est la différence entre `0..5` et `0..=5` ?
6. Comment convertir un `i32` en `f64` ?

→ [Chapitre 2 : Ownership et emprunts](../02-ownership-et-emprunts/README.md)
