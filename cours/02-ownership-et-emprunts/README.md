# Chapitre 2 — Ownership, emprunts et slices

**Le chapitre qui compte.** Tout ce qui rend Rust différent est ici : la sécurité mémoire sans
ramasse-miettes. Prévois deux passages. Si tu ne comprends que 70 % du chapitre 2, tu passeras les
chapitres suivants à te battre contre le compilateur ; si tu le comprends, le reste du langage
devient presque facile.

---

## 1. Le problème que ça résout

Dans un langage à ramasse-miettes (Java, Python, JS), la mémoire est libérée « plus tard, par
quelqu'un d'autre ». En C, tu la libères à la main, et tu as trois façons classiques de te tromper :
libérer deux fois, utiliser après libération, oublier de libérer.

Rust choisit une troisième voie : **des règles vérifiées à la compilation**. Zéro coût à l'exécution,
zéro ramasse-miettes, et ces bugs deviennent impossibles à écrire.

Les trois règles de l'ownership :

1. Chaque valeur a **un** propriétaire (owner).
2. Il ne peut y avoir qu'un seul propriétaire à la fois.
3. Quand le propriétaire sort de portée, la valeur est **libérée** (`drop`).

```rust
{
    let s = String::from("bonjour");   // s est propriétaire
    // ... on utilise s
}                                       // fin de portée → mémoire libérée automatiquement
```

---

## 2. Pile et tas (il faut le savoir)

- La **pile** (stack) : rapide, taille connue à la compilation. `i32`, `bool`, `char`, `f64`,
  les tableaux `[T; N]`, les tuples de types simples.
- Le **tas** (heap) : pour ce dont la taille varie à l'exécution. `String`, `Vec<T>`, `Box<T>`.

Une `String` est en réalité, sur la pile, un triplet `(pointeur, longueur, capacité)` qui pointe
vers des octets sur le tas. C'est ce triplet qu'on copie ou qu'on déplace.

---

## 3. Move : l'affectation déplace

```rust
let a = String::from("salut");
let b = a;            // MOVE : a n'est plus utilisable
// println!("{a}");   // ERREUR E0382: borrow of moved value
```

Pourquoi ? Si `a` et `b` pointaient vers les mêmes octets, la fin de portée les libérerait **deux
fois**. Rust invalide donc la source. Ce n'est pas une copie coûteuse — seul le triplet est recopié —
mais la source devient inutilisable.

### `Copy` : l'exception

```rust
let x = 5;
let y = x;
println!("{x}");   // OK ! i32 implémente Copy
```

Les types entièrement sur la pile et bon marché à dupliquer implémentent le trait `Copy` :
tous les nombres, `bool`, `char`, et les tuples/tableaux dont tous les éléments sont `Copy`.
Pour eux, l'affectation copie au lieu de déplacer.

**Règle mentale :** « ça a un `String`/`Vec` dedans → ça se déplace ; c'est un nombre → ça se copie. »

### Move et fonctions

Passer une valeur à une fonction, c'est la déplacer :

```rust
fn consomme(s: String) { }      // s meurt ici

let texte = String::from("hop");
consomme(texte);
// texte est mort. Utilisation → E0382
```

Sauf si la fonction te la rend (`-> String`). Mais rendre systématiquement les valeurs serait
insupportable. D'où : les emprunts.

### `clone()`

```rust
let b = a.clone();     // duplication explicite, incluant les données du tas
```

`clone()` résout tout… et coûte une allocation. Pendant les premiers chapitres, **cloner pour
avancer est acceptable**. Mais chaque `clone()` doit être une décision consciente, pas un réflexe
pour faire taire le compilateur. Une bonne partie du chapitre 8 consiste à apprendre à les enlever.

---

## 4. Les emprunts (borrowing)

Emprunter = accéder à une valeur **sans en prendre possession**.

```rust
fn longueur(s: &String) -> usize {   // &  = référence partagée
    s.len()
}                                     // rien n'est libéré ici : on ne possédait pas

let texte = String::from("bonjour");
let n = longueur(&texte);
println!("{texte} fait {n}");         // texte est toujours vivant
```

```rust
fn crier(s: &mut String) {            // &mut = référence exclusive
    s.push_str(" !!!");
}

let mut texte = String::from("hop");
crier(&mut texte);
```

### **LA règle du borrow checker**

À un instant donné, pour une valeur donnée, tu peux avoir :

- soit **autant de références partagées (`&T`) que tu veux** ;
- soit **une seule référence exclusive (`&mut T`)** ;
- **jamais les deux en même temps.**

Et toute référence doit toujours pointer vers une valeur vivante (pas de référence pendouillante).

C'est exactement le théorème « aliasing XOR mutation » : si personne d'autre ne regarde, tu peux
modifier ; si d'autres regardent, personne ne modifie. C'est ce qui rend les data races impossibles
(chapitre 11) et ce qui autorise des optimisations agressives.

```rust
let mut v = vec![1, 2, 3];
let r1 = &v;
let r2 = &v;          // ok, deux lecteurs
println!("{r1:?} {r2:?}");   // dernière utilisation de r1 et r2

let r3 = &mut v;      // ok ! r1/r2 ne servent plus après cette ligne
r3.push(4);
```

Ce dernier point s'appelle le **NLL** (non-lexical lifetimes) : un emprunt s'arrête à sa **dernière
utilisation**, pas à la fin du bloc. Beaucoup d'erreurs se règlent en déplaçant une ligne.

---

## 5. Slices : `&str` et `&[T]`

Une slice est **une vue empruntée sur une portion contiguë** : un pointeur + une longueur.
Elle ne possède rien.

```rust
let phrase = String::from("bonjour le monde");
let debut: &str = &phrase[0..7];       // "bonjour"
let tout: &str = &phrase;              // deref coercion : &String → &str

let notes = vec![10, 12, 14, 16];
let trois_premieres: &[i32] = &notes[..3];
```

### `String` vs `&str` — le duo à intégrer une fois pour toutes

| | `String` | `&str` |
|---|---|---|
| Possède ses données | oui | non |
| Sur le tas | oui | pointe vers (tas, binaire, ou pile) |
| Modifiable | oui (`push_str`, `push`) | non |
| Littéral `"abc"` | non | oui (`&'static str`) |

**Règle pratique :**
- en **paramètre** de fonction, prends `&str` (accepte un littéral *et* une `String`) ;
- en **champ de struct** ou en **retour**, possède une `String` (au moins jusqu'au chapitre 8).

La même dualité existe partout : `String`/`&str`, `Vec<T>`/`&[T]`, `PathBuf`/`&Path`.
C'est le motif « propriétaire / vue empruntée », omniprésent en Rust.

### Attention à l'UTF-8

Une `String` Rust est de l'UTF-8 valide. Donc :

- `s.len()` renvoie un nombre d'**octets**, pas de caractères. `"héhé".len()` vaut 6.
- `s[0]` n'existe pas. Indexer par octet pourrait couper un caractère en deux.
- `&s[0..3]` **panique** si 3 tombe au milieu d'un caractère.
- Pour parcourir : `s.chars()` (par caractère), `s.bytes()` (par octet),
  `s.char_indices()` (les deux).

---

## 6. Lire les erreurs d'emprunt

Les quatre que tu vas rencontrer :

| Code | Message | Traduction | Réflexe |
|------|---------|------------|---------|
| `E0382` | use of moved value | tu utilises une valeur déplacée | emprunter (`&`) au lieu de déplacer, ou `clone()` |
| `E0499` | cannot borrow as mutable more than once | deux `&mut` simultanés | réduire la portée, séparer les opérations |
| `E0502` | cannot borrow as mutable because also borrowed as immutable | un lecteur est encore vivant | déplacer l'usage du lecteur avant, ou copier la valeur lue |
| `E0106` | missing lifetime specifier | tu renvoies une référence sans dire d'où elle vient | chapitre 8 — pour l'instant, renvoie une valeur possédée |

Cas d'école de `E0502`, que tu vas *forcément* rencontrer :

```rust
let mut v = vec![1, 2, 3];
let premier = &v[0];    // emprunt partagé
v.push(4);              // ERREUR : push a besoin de &mut, et `premier` vit encore
println!("{premier}");
```

Ce n'est pas du zèle : `push` peut réallouer le tas, ce qui rendrait `premier` pendouillant.
En C++, ce code compile et lit de la mémoire libérée. Solution ici : `let premier = v[0];`
(copie de l'`i32`) — l'emprunt s'arrête immédiatement.

---

## Exercices

### 2.1 — `ch02-move-ou-pas` (lecture, pas écriture)

Crée le projet et, dans `main.rs`, écris **six** petits blocs de code que tu penses fautifs
(inspirés du chapitre : move puis usage, deux `&mut`, `&` + `&mut`, référence vers une valeur
morte, `s[0]` sur une String, `&s[0..1]` au milieu d'un « é »).

Pour chacun : mets-le en commentaire avec, au-dessus, **ta prédiction** du code d'erreur.
Puis décommente un par un et vérifie avec `cargo check`.

**Objectif :** apprendre à *anticiper* le compilateur. C'est l'exercice le plus rentable du cours,
ne le saute pas parce qu'il ne « produit » rien.

**Réussi si :** tu as prédit correctement au moins 4 erreurs sur 6, et que tu peux expliquer les 2 autres.

---

### 2.2 — `ch02-mots`

Écris une bibliothèque de fonctions sur du texte, appelées depuis `main` :

```rust
fn premier_mot(phrase: &str) -> &str
fn compte_mots(phrase: &str) -> usize
fn plus_long_mot(phrase: &str) -> &str
fn inverser_mots(phrase: &str) -> String     // "un deux trois" -> "trois deux un"
fn compte_voyelles(phrase: &str) -> usize    // gère é, à, ù
```

**Contraintes :**
- les trois premières renvoient une **slice** de l'entrée : aucune allocation, aucun `to_string()` ;
- `inverser_mots` renvoie une `String` (elle construit du neuf, elle doit posséder) ;
- teste avec des phrases contenant des accents et des espaces multiples.

**Indices :** `split_whitespace()`, `chars()`, `to_lowercase()`, `contains()`. Pour renvoyer une
slice, tu peux indexer par octets — mais réfléchis d'abord à ce que te renvoient déjà les
itérateurs de `str`.

**Question à te poser :** pourquoi `fn premier_mot(phrase: &str) -> &str` compile sans annotation
de durée de vie, alors que `fn choisir(a: &str, b: &str) -> &str` ne compilera pas ? (Réponse au
chapitre 8 — mais essaie d'écrire la seconde et lis l'erreur maintenant.)

---

### 2.3 — `ch02-inventaire`

Modélise un inventaire simple avec un `Vec<String>` dans `main`, et écris :

```rust
fn ajouter(inventaire: &mut Vec<String>, objet: &str)
fn contient(inventaire: &[String], objet: &str) -> bool
fn retirer(inventaire: &mut Vec<String>, objet: &str) -> bool   // true si retiré
fn afficher(inventaire: &[String])
fn fusionner(a: Vec<String>, b: Vec<String>) -> Vec<String>     // consomme les deux !
```

**Contraintes :**
- respecte scrupuleusement les types de paramètres donnés : `&[String]` pour lire, `&mut Vec<String>`
  pour modifier, `Vec<String>` (par valeur) pour consommer ;
- dans `main`, après avoir appelé `fusionner(a, b)`, essaie d'utiliser `a`. Lis l'erreur. Comprends-la.
- `fusionner` doit trier le résultat et supprimer les doublons (`sort`, `dedup`).

**Indices :** `iter().any()`, `position()`, `remove()`, `extend()`. Attention : `remove(i)` prend un
index `usize`, et `position` renvoie une `Option<usize>` — le chapitre 4 formalisera `Option`,
mais un `if let Some(i) = ...` suffit dès maintenant.

---

### 2.4 — `ch02-borrow-puzzle`

Je te donne quatre situations. Pour chacune, écris le code *fautif* d'abord (il ne compile pas),
puis **corrige-le de deux façons différentes** en commentant ce que chaque correction coûte
(allocation ? restructuration ? copie ?).

1. Tu veux ajouter à un `Vec` la première valeur qu'il contient (doubler le premier élément).
2. Tu veux modifier un élément d'un `Vec` pendant que tu itères dessus.
3. Tu veux qu'une fonction renvoie la plus longue de deux `String` passées en référence.
4. Tu veux stocker dans une variable une référence vers une `String` créée dans un bloc `{}`.

**Réussi si :** pour chaque cas, tu sais dire en une phrase *quelle règle* était violée.

---

### 2.5 — `ch02-cesar` (synthèse)

Implémente un chiffre de César :

```rust
fn chiffrer(texte: &str, decalage: u8) -> String
fn dechiffrer(texte: &str, decalage: u8) -> String
fn casser(texte: &str) -> (u8, String)   // teste les 26 décalages, renvoie le plus probable
```

**Contraintes :**
- seules les lettres a–z et A–Z sont décalées ; la ponctuation, les chiffres et les espaces passent
  intacts ; la casse est préservée ;
- `casser` choisit le décalage dont le texte résultant contient le plus de mots français fréquents
  (garde une petite liste en dur : `le`, `la`, `les`, `de`, `et`, `un`, `une`, `est`…) ;
- `dechiffrer` doit pouvoir être écrite en une ligne à partir de `chiffrer`.

**Indices :** `char::is_ascii_alphabetic`, `as u8` / `as char`, arithmétique modulo 26 en partant de
`b'a'` ou `b'A'`. Attention aux débordements de `u8` : passe par `u32` ou utilise `%` avant de
réadditionner.

**Réussi si :** `dechiffrer(&chiffrer("Bonjour, le monde !", 13), 13)` redonne l'original exactement,
accents compris.

---

## ✅ Auto-évaluation

1. Énonce les trois règles de l'ownership.
2. Énonce la règle du borrow checker en une phrase.
3. Pourquoi `let b = a;` invalide `a` quand `a` est une `String` mais pas quand c'est un `i32` ?
4. Quand mettre `&str` et quand mettre `String` dans une signature ?
5. Pourquoi `v.push(x)` peut-il invalider une référence vers `v[0]` ?
6. Que vaut `"héhé".len()` et pourquoi ?

Si tu hésites sur la 2 ou la 5, relis avant de continuer. Vraiment.

→ [Chapitre 3 : Structs, enums, pattern matching](../03-structs-enums-pattern-matching/README.md)
