# Chapitre 6 — Traits et génériques

**Objectif :** écrire du code qui marche pour plusieurs types, et définir des comportements
interchangeables. C'est le système d'abstraction de Rust — ni héritage, ni interfaces au sens
Java : quelque chose de plus proche des *typeclasses*.

---

## 1. Génériques

```rust
fn plus_grand<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Paire<T> { gauche: T, droite: T }

impl<T: std::fmt::Display> Paire<T> {
    fn afficher(&self) { println!("({}, {})", self.gauche, self.droite); }
}
```

`T: PartialOrd` est une **borne** (trait bound) : « T, quel qu'il soit, doit savoir se comparer ».
Sans elle, `a > b` ne compile pas — le générique Rust est vérifié **à la définition**, pas à
l'usage (contrairement aux templates C++, d'où des messages d'erreur bien plus lisibles).

Coût à l'exécution : **zéro**. Le compilateur fait de la *monomorphisation* : il génère une copie
spécialisée de la fonction pour chaque type réellement utilisé.

Syntaxe des bornes multiples :

```rust
fn traiter<T: Clone + std::fmt::Debug>(x: T) { }

fn traiter<T>(x: T) -> String
where
    T: Clone + std::fmt::Debug,       // clause `where` : préférée dès que ça se complique
{ }
```

---

## 2. Traits

Un trait décrit un **comportement** que des types peuvent implémenter.

```rust
trait Scheduler {
    /// Nom lisible de l'algorithme.
    fn name(&self) -> &str;

    /// Calcule le nouvel état d'une carte après une réponse.
    fn next_state(&self, state: &CardState, grade: Grade) -> CardState;

    /// Méthode avec implémentation PAR DÉFAUT : optionnelle à l'implémentation.
    fn describe(&self) -> String {
        format!("algorithme « {} »", self.name())
    }
}

struct Sm2 { facilite_min: f32 }

impl Scheduler for Sm2 {
    fn name(&self) -> &str { "SM-2" }
    fn next_state(&self, state: &CardState, grade: Grade) -> CardState { ... }
}
```

**Règle d'orphelin :** tu peux implémenter *ton* trait pour *n'importe quel* type, ou *n'importe
quel* trait pour *ton* type — mais pas un trait externe pour un type externe (pas d'`impl Display
for Vec<T>`). Le contournement : le newtype du chapitre 3.

### Les traits standard à connaître

| Trait | Rôle | Comment l'obtenir |
|-------|------|-------------------|
| `Debug` | `{:?}` | `#[derive(Debug)]` |
| `Display` | `{}` — la sortie « pour l'utilisateur » | **à écrire à la main** |
| `Clone` / `Copy` | duplication | derive |
| `Default` | valeur par défaut | derive ou manuel |
| `PartialEq`/`Eq`/`PartialOrd`/`Ord` | comparaison, tri | derive ou manuel |
| `From` / `Into` | conversion infaillible | implémente `From`, tu obtiens `Into` gratuitement |
| `TryFrom` / `TryInto` | conversion faillible (`Result`) | manuel |
| `FromStr` | `"...".parse::<T>()` | manuel — très satisfaisant à implémenter |
| `Iterator` | boucle `for` | chapitre 7 |
| `Error` | erreur standard | manuel (ou `thiserror`) |
| `Drop` | code au moment de la libération | manuel, rare |

`Display` s'écrit ainsi :

```rust
use std::fmt;

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self { Grade::Again => "oubliée", /* ... */ };
        write!(f, "{s}")
    }
}
```

Bonus : implémenter `Display` te donne `.to_string()` gratuitement (via le trait `ToString`).

`From` est le trait le plus rentable du langage :

```rust
impl From<std::io::Error> for MemoError {
    fn from(e: std::io::Error) -> Self { MemoError::Io(e) }
}
```

…et d'un coup, `?` convertit automatiquement toutes tes erreurs d'IO. C'est le mécanisme derrière
la magie du chapitre 4.

---

## 3. Polymorphisme statique vs dynamique

```rust
fn reviser(s: &impl Scheduler) { }          // statique : un type connu à la compilation
fn reviser<S: Scheduler>(s: &S) { }         // identique, forme explicite

fn reviser(s: &dyn Scheduler) { }           // dynamique : type résolu à l'exécution
let algos: Vec<Box<dyn Scheduler>> = vec![Box::new(Sm2::new()), Box::new(Leitner::new())];
```

| | `impl Trait` / `<T: Trait>` | `dyn Trait` |
|---|---|---|
| Résolution | compilation (monomorphisation) | exécution (vtable) |
| Coût | nul | un pointeur d'indirection |
| Taille du binaire | plus gros | plus petit |
| Peut-on mélanger plusieurs types dans un `Vec` ? | non | **oui** |

**Règle pratique :** générique par défaut ; `dyn` quand tu as besoin d'hétérogénéité (une collection
de choses différentes) ou de choisir l'implémentation à l'exécution (une option de configuration).

Pour Mémo : `Box<dyn Scheduler>` te permettra de choisir l'algorithme selon un argument CLI.

---

## 4. Traits génériques et types associés

```rust
trait Depot {
    type Item;                                    // type ASSOCIÉ
    fn charger(&self) -> Result<Vec<Self::Item>, MemoError>;
    fn sauver(&self, items: &[Self::Item]) -> Result<(), MemoError>;
}
```

Différence avec un paramètre générique `trait Depot<T>` : avec un type associé, un type ne peut
implémenter le trait **qu'une fois** (un `JsonStore` a un seul `Item`). Avec un générique, il peut
l'implémenter plusieurs fois (`From<i32>` et `From<&str>` pour le même type). Choisis selon que
« plusieurs implémentations pour un même type » a du sens ou non.

`Iterator` utilise un type associé (`type Item`) — c'est pourquoi tu écris `Iterator<Item = u32>`
avec un `=`.

---

## 5. Le mot sur les `impl` génériques conditionnels

```rust
impl<T: Display> Paire<T> {
    fn afficher(&self) { }      // cette méthode n'existe QUE si T: Display
}
```

Ça s'appelle une *blanket implementation* quand c'est appliqué largement, et c'est comme ça que
la stdlib te donne `.to_string()` sur tout ce qui est `Display`. C'est de la composition très fine :
un type gagne des capacités en fonction de ce qu'il sait déjà faire.

---

## Exercices

### 6.1 — `ch06-affichage`

Reprends `ch03-formes`. Implémente à la main, sans derive :

- `Display` pour `Forme` : `Cercle(r=2.5)`, `Rectangle(3×4)` ;
- `PartialOrd` + `Ord`… et découvre que c'est impossible sur des `f64`. Comprends pourquoi
  (`NaN` casse la totalité de l'ordre), puis contourne : trie avec
  `sort_by(|a, b| a.aire().partial_cmp(&b.aire()).unwrap())`, et explique en commentaire pourquoi
  cet `unwrap` est acceptable ici ;
- `Default` ;
- `FromStr` pour parser `"cercle 2.5"` et `"rect 3 4"` → `"cercle 2.5".parse::<Forme>()` doit marcher ;
- `From<f64> for Forme` (un flottant devient un cercle de ce rayon).

**Réussi si :** dans `main`, `println!("{}", forme)` et `"rect 3 4".parse::<Forme>()?` fonctionnent.

---

### 6.2 — `ch06-conteneur`

Écris une structure générique `Pile<T>` (LIFO) avec capacité maximale :

```rust
struct Pile<T> { elements: Vec<T>, capacite: usize }

impl<T> Pile<T> {
    fn nouvelle(capacite: usize) -> Self
    fn empiler(&mut self, x: T) -> Result<(), PileError>
    fn depiler(&mut self) -> Option<T>
    fn sommet(&self) -> Option<&T>
    fn taille(&self) -> usize
    fn est_pleine(&self) -> bool
}

impl<T: Clone> Pile<T> { fn dupliquer_sommet(&mut self) -> Result<(), PileError> }
impl<T: PartialEq> Pile<T> { fn contient(&self, x: &T) -> bool }
impl<T: std::fmt::Debug> std::fmt::Display for Pile<T> { }
```

**Contrainte clé :** remarque que `Pile<T>` de base n'exige **rien** de `T`, et que les capacités
supplémentaires sont conditionnées par des bornes. Teste avec `Pile<i32>`, `Pile<String>`, et un
`Pile<Forme>`.

**Bonus :** implémente `IntoIterator` pour `Pile<T>` afin que `for x in pile` fonctionne.

---

### 6.3 — `ch06-animaux-dyn`

Un trait `Animal` avec `nom()`, `cri()`, et une méthode par défaut `presenter()` qui combine les
deux. Trois implémentations. Puis :

```rust
fn concert(animaux: &[Box<dyn Animal>])
fn le_plus_bruyant<'a>(animaux: &'a [Box<dyn Animal>]) -> &'a dyn Animal
```

**Contraintes :**
- ajoute une méthode `fn pattes(&self) -> u8` avec une valeur par défaut de 4, et un serpent qui la
  redéfinit à 0 ;
- essaie ensuite d'écrire `fn concert<A: Animal>(animaux: &[A])` et de lui passer le mélange :
  ça ne compile pas. **Écris en commentaire pourquoi.** C'est le point pédagogique de l'exercice.

---

### 6.4 — `ch06-scheduler` 🎯 **JALON MÉMO 4/9**

Le cœur intelligent de Mémo. Dans `projet/memo`, crée `src/scheduler.rs`.

**Le trait :**

```rust
pub trait Scheduler {
    fn name(&self) -> &str;
    /// Calcule le nouvel état d'une carte après que l'utilisateur s'est auto-noté.
    fn next(&self, state: &CardState, grade: Grade) -> CardState;
    fn describe(&self) -> String { /* défaut */ }
}
```

**Deux implémentations :**

1. **`Leitner`** — le système des boîtes, simple. Cinq boîtes d'intervalles 1, 3, 7, 14, 30 jours.
   Bonne réponse → boîte suivante. Mauvaise réponse → retour boîte 1.

2. **`Sm2`** — l'algorithme de SuperMemo 2, celui d'Anki. Sa spécification :

   - chaque carte porte un **facteur de facilité** `EF` (départ 2.5, plancher 1.3) et un
     **intervalle** en jours ;
   - la note `q` va de 0 à 3 dans notre modèle (oubliée, difficile, correcte, facile) —
     l'algorithme original va de 0 à 5, tu devras faire une correspondance et **la documenter** ;
   - si la réponse est « oubliée » : la carte repart en apprentissage, intervalle remis à 1 jour,
     `EF` diminué ;
   - sinon : 1re réussite → 1 jour, 2e → 6 jours, ensuite → `intervalle × EF` arrondi ;
   - mise à jour du facteur : `EF' = EF + (0.1 − (3 − q) × (0.08 + (3 − q) × 0.02))`,
     borné en bas à 1.3 ;
   - une note « facile » peut appliquer un bonus multiplicatif (à toi de choisir, documente-le).

**Contraintes :**
- `next` est **pure** : elle ne mute rien, elle prend un état et renvoie un nouvel état.
  C'est ce qui la rend triviale à tester ;
- `Library` reçoit une méthode
  `fn review(&mut self, id: u32, grade: Grade, scheduler: &dyn Scheduler) -> Result<&Card, MemoError>` ;
- écris **au moins 12 tests** sur `Sm2`, dont : une carte neuve notée « correcte » passe à 1 jour ;
  la même deux fois de suite → 6 jours ; l'`EF` ne descend jamais sous 1.3 même après 20 échecs ;
  un échec remet l'intervalle à 1 mais **ne remet pas** l'`EF` à 2.5 ; l'intervalle croît
  strictement après plusieurs bonnes réponses ;
- implémente `Display` pour `Grade` et pour `CardState` (affichage lisible du type
  « planifiée, 6 j, EF 2.36 »).

**Question de conception à trancher :** `Library::review` prend-elle `&dyn Scheduler` ou un
générique `<S: Scheduler>` ? Les deux marchent. Écris trois lignes de justification en commentaire.
(Indice : que veux-tu faire quand l'utilisateur passe `--algo leitner` en ligne de commande ?)

---

## ✅ Auto-évaluation

1. Différence entre `impl Trait` en paramètre et `dyn Trait` ?
2. Pourquoi ne peux-tu pas mettre `Sm2` et `Leitner` dans le même `Vec<S>` générique ?
3. Qu'est-ce que la règle d'orphelin ?
4. Qu'obtiens-tu gratuitement en implémentant `Display` ? Et `From` ?
5. À quoi sert un type associé plutôt qu'un paramètre générique ?
6. La monomorphisation : qu'est-ce que c'est, et quel est son coût ?

→ [Chapitre 7 : Closures et itérateurs](../07-closures-et-iterateurs/README.md)
