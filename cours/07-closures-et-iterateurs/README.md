# Chapitre 7 — Closures et itérateurs

**Objectif :** écrire du Rust idiomatique. C'est le chapitre le plus gratifiant : des boucles de
quinze lignes deviennent des chaînes de trois, sans perte de performance.

---

## 1. Closures

Une closure est une fonction anonyme qui **capture** son environnement.

```rust
let seuil = 10;
let au_dessus = |x: i32| x > seuil;      // capture `seuil`
println!("{}", au_dessus(12));

let doubler = |x| x * 2;                  // types inférés à la première utilisation
let long = |x: i32| -> i32 {
    let y = x * 2;
    y + 1
};
```

### Les trois traits de closure

Selon ce que la closure fait de son environnement, elle implémente automatiquement :

| Trait | La closure… | Peut être appelée |
|-------|-------------|-------------------|
| `Fn` | lit ses captures (`&T`) | plusieurs fois |
| `FnMut` | modifie ses captures (`&mut T`) | plusieurs fois, mais exclusive |
| `FnOnce` | consomme ses captures (`T`) | **une seule fois** |

Tu ne les écris jamais toi-même : le compilateur choisit le plus permissif possible. Tu les
rencontres dans les **signatures** :

```rust
fn appliquer<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn repeter<F: FnMut()>(mut f: F, n: usize) { for _ in 0..n { f(); } }
```

### `move`

```rust
let nom = String::from("Alice");
let saluer = move || println!("Bonjour {nom}");   // la closure PREND POSSESSION de `nom`
// nom n'est plus utilisable ici
```

`move` force la capture par valeur. Indispensable quand la closure survit à la portée courante :
threads (chapitre 11), callbacks stockés, valeurs renvoyées.

### Renvoyer une closure

```rust
fn multiplicateur(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn choisir(mode: &str) -> Box<dyn Fn(i32) -> i32> {   // si plusieurs types possibles
    match mode {
        "double" => Box::new(|x| x * 2),
        _        => Box::new(|x| x + 1),
    }
}
```

Chaque closure a un type **anonyme et unique**, d'où `impl Fn` ou `Box<dyn Fn>`.

---

## 2. Itérateurs

Le trait `Iterator` tient en une méthode :

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...et ~75 méthodes fournies gratuitement par-dessus
}
```

Les itérateurs sont **paresseux** : rien ne se passe tant qu'on ne consomme pas.
`v.iter().map(...)` ne calcule rien ; c'est le `collect()`, le `sum()` ou le `for` final qui tire.

Clippy te dira `unused Iterator` si tu oublies de consommer — écoute-le.

### Adaptateurs (renvoient un itérateur)

```rust
.map(|x| x * 2)
.filter(|x| **x > 3)              // attention aux niveaux de référence !
.filter_map(|s| s.parse::<i32>().ok())   // filtre + transforme d'un coup
.flat_map(|s| s.chars())
.flatten()                        // aplatit Vec<Vec<T>> ou enlève les None
.take(5) / .skip(2)
.take_while(|x| *x < 10) / .skip_while(...)
.step_by(2)
.chain(autre_iter)
.zip(autre_iter)                  // paires, s'arrête au plus court
.enumerate()                      // (index, valeur)
.rev()                            // à l'envers (si DoubleEndedIterator)
.peekable()                       // permet .peek()
.inspect(|x| println!("{x:?}"))   // débogage sans casser la chaîne
.cloned() / .copied()             // &T -> T
.scan(état, |acc, x| ...)         // fold qui produit un itérateur
.windows(3) / .chunks(3)          // sur les slices, pas les itérateurs
```

### Consommateurs (terminent la chaîne)

```rust
.collect::<Vec<_>>()              // ou : let v: Vec<_> = ....collect();
.sum::<i32>() / .product()
.count() / .last() / .nth(3)
.min() / .max() / .min_by_key(|c| c.id) / .max_by(|a, b| ...)
.fold(0, |acc, x| acc + x)        // le couteau suisse
.reduce(|a, b| ...)               // fold sans valeur initiale → Option
.any(|x| ...) / .all(|x| ...)     // court-circuitent
.find(|x| ...) / .position(|x| ...)
.for_each(|x| ...)
.partition(|x| ...)               // (ceux qui oui, ceux qui non)
.unzip()
```

### `collect` est magique

`collect()` peut construire presque n'importe quoi, selon le type demandé :

```rust
let v: Vec<i32>                = iter.collect();
let s: String                  = iter.collect();          // sur un itérateur de char
let m: HashMap<String, u32>    = iter.collect();          // sur un itérateur de paires
let s: HashSet<u32>            = iter.collect();
let r: Result<Vec<i32>, _>     = iter.collect();          // ← LE truc à connaître
```

La dernière ligne est un bijou : un itérateur de `Result<T, E>` se collecte en
`Result<Vec<T>, E>` — le premier `Err` court-circuite tout. Le motif idéal pour parser un fichier
ligne par ligne en échouant proprement à la première ligne invalide. Idem avec `Option`.

### Créer un itérateur

```rust
(1..=10)                          // Range
std::iter::repeat(0).take(5)
std::iter::successors(Some(1), |n| Some(n * 2)).take(10)   // 1,2,4,8...
std::iter::once(x) / std::iter::empty()
```

Ou implémente `Iterator` toi-même : une struct qui garde l'état + `fn next(&mut self)`.
Tu obtiens alors les 75 méthodes gratuitement sur ton type. Fais-le au moins une fois (exercice 7.3).

### Performance

Une chaîne d'itérateurs compile vers **le même code machine** qu'une boucle écrite à la main
(« zero-cost abstraction »). En release, `v.iter().filter(...).map(...).sum()` est aussi rapide
qu'un `for`. Écris la version lisible.

---

## 3. Le piège des niveaux de référence

L'erreur qui rend fou au début :

```rust
let v = vec![1, 2, 3];
v.iter().filter(|x| *x > &1)      // x est &&i32 dans filter !
v.iter().filter(|&&x| x > 1)      // déstructuration dans le motif : plus lisible
v.iter().copied().filter(|x| *x > 1)   // ou aplatis d'abord
```

Pourquoi ? `iter()` produit des `&i32`, et `filter` passe une **référence** à son prédicat →
`&&i32`. Trois solutions : déréférencer (`*x`), déstructurer dans le motif (`|&&x|`), ou
`.copied()`/`.cloned()` en amont. La deuxième est la plus idiomatique.

Rappel des trois `iter` :

| Méthode | Item | Effet sur la collection |
|---------|------|-------------------------|
| `.iter()` | `&T` | inchangée |
| `.iter_mut()` | `&mut T` | modifiable en place |
| `.into_iter()` | `T` | consommée |

---

## Exercices

### 7.1 — `ch07-reecriture`

Reprends **tel quel** ton exercice `ch01-statistiques` (où les boucles étaient imposées) et réécris
chaque fonction en une seule expression d'itérateur. Garde les deux versions côte à côte
(`moyenne_boucle` / `moyenne_iter`) et un test qui vérifie qu'elles donnent le même résultat.

Ajoute ensuite : médiane, écart-type, mode (valeur la plus fréquente), et le top 3 des notes.

**Puis mesure :** compile en `--release` et chronomètre les deux versions sur un `Vec` d'un million
d'éléments (`std::time::Instant`). Note le résultat en commentaire. C'est ta preuve expérimentale du
« zero-cost ».

---

### 7.2 — `ch07-pipeline-csv`

On te donne une constante `DONNEES: &str` contenant du CSV brut :

```
nom,age,ville,salaire
Alice,30,Paris,45000
Bob,x,Lyon,38000
Charlie,45,Paris,62000
```

Écris **une seule chaîne d'itérateurs** (pas de `for`, pas de `mut`) qui produit
`Result<Vec<Personne>, ErreurParse>` — donc qui échoue proprement sur la ligne de Bob avec le
numéro de ligne.

Puis, à partir du `Vec<Personne>` :
- salaire moyen par ville → `HashMap<String, f64>` ;
- les 3 mieux payés, par ordre décroissant ;
- la personne la plus âgée de chaque ville ;
- le pourcentage de personnes de plus de 40 ans, à une décimale.

**Contraintes :** `lines()`, `skip(1)`, `enumerate()` pour le numéro de ligne, `split(',')`,
`collect::<Result<Vec<_>, _>>()`. Aucune boucle `for` dans tout le fichier — c'est artificiel,
mais c'est l'objet de l'exercice.

**Indice pour les moyennes par groupe :** `fold` avec une `HashMap<String, (f64, usize)>`
(somme, compte), puis un second `map` pour diviser.

---

### 7.3 — `ch07-fibonacci-iter`

Implémente `Iterator` à la main sur une struct `Fibonacci`, puis prouve que tu en tires tout
l'écosystème :

- les 20 premiers termes ;
- les termes pairs sous 1 000 000, sommés ;
- le premier terme à dépasser `u32::MAX` (indice : `u64`, ou `checked_add` et `take_while`) ;
- le rapport `f(n+1)/f(n)` des 15 premiers termes (approche du nombre d'or) — utilise `zip` avec
  une version décalée, ou `scan`.

**Bonus :** écris un itérateur `Collatz` (suite de Syracuse) et trouve, en une chaîne, l'entier
sous 10 000 dont la suite est la plus longue.

---

### 7.4 — `ch07-stats-memo` 🎯 **JALON MÉMO 5/9**

Dans `projet/memo`, crée `src/stats.rs` et enrichis `Library` — **tout en style itérateur**.

```rust
pub struct DeckStats {
    pub deck: String,
    pub total: usize,
    pub new_cards: usize,
    pub learning: usize,
    pub scheduled: usize,
    pub suspended: usize,
    pub due_today: usize,
    pub maturity: f32,        // % de cartes dont l'intervalle >= 21 jours
    pub avg_ease: Option<f32>,
}

impl Library {
    pub fn stats(&self, deck: &str) -> Option<DeckStats>
    pub fn all_stats(&self) -> Vec<DeckStats>            // trié par nom de paquet
    pub fn search(&self, motif: &str) -> Vec<&Card>      // insensible à la casse, Q et R
    pub fn due_queue(&self, deck: Option<&str>, limite: usize) -> Vec<&Card>
    pub fn hardest(&self, n: usize) -> Vec<&Card>        // les n plus faibles facteurs de facilité
}
```

**Contraintes :**
- **aucune boucle `for`** dans `stats.rs` ;
- `due_queue` mélange l'ordre (les cartes ne doivent pas toujours sortir dans le même sens) mais
  reste **déterministe si on lui passe une graine** — pense à l'avenir : tu voudras tester cette
  fonction. Choisis ta stratégie et documente-la ;
- `maturity` sur un paquet vide vaut 0.0 et ne divise pas par zéro ;
- `avg_ease` est `None` s'il n'y a aucune carte planifiée ;
- tests : un paquet vide, un paquet à une carte, un paquet mixte dont les chiffres sont vérifiés
  à la main.

**Puis, dans `main.rs`, une barre de progression texte :**

```
rust     ████████████░░░░░░  62 cartes  (maturité 71 %)
```

**Contrainte piège :** la largeur de la barre doit être constante et calculée proprement — attention
au fait qu'un caractère `█` fait 3 octets en UTF-8. Si tu utilises `len()` pour aligner, ton tableau
sera de travers. (`chars().count()` compte les caractères ; la largeur d'affichage réelle est encore
un autre problème, cherche « largeur de colonne unicode » si tu veux creuser.)

---

## ✅ Auto-évaluation

1. Différence entre `Fn`, `FnMut`, `FnOnce` ?
2. Que fait `move` devant une closure et quand en as-tu besoin ?
3. Pourquoi `v.iter().map(f)` seul ne fait-il rien ?
4. Comment collecter un itérateur de `Result` en `Result<Vec<_>, _>` et pourquoi c'est utile ?
5. Pourquoi `|x| *x > 1` dans un `filter` sur `.iter()` ?
6. Une chaîne d'itérateurs est-elle plus lente qu'une boucle `for` ?

→ [Chapitre 8 : Durées de vie et smart pointers](../08-lifetimes-et-smart-pointers/README.md)
