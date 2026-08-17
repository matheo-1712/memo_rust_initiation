# Chapitre 4 — Collections et gestion d'erreurs

**Objectif :** manipuler des `Vec`, `HashMap`, `HashSet` couramment, et surtout **arrêter de faire
planter tes programmes** avec `unwrap()`. La gestion d'erreurs de Rust est une de ses plus belles
réussites : elle est explicite sans être verbeuse.

---

## 1. `Vec<T>`

Un tableau qui grandit, sur le tas.

```rust
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];            // macro pratique
let v = vec![0u8; 1024];              // 1024 zéros

v.push(4);
let dernier = v.pop();                // Option<i32> : None si vide
v.insert(0, 42);
v.remove(0);                          // panique si l'index n'existe pas
v.retain(|x| *x > 0);                 // garde ceux qui satisfont le prédicat
v.sort();
v.sort_by_key(|c| c.id);
v.dedup();
v.clear();

let n = v.len();
if v.is_empty() { }
if v.contains(&42) { }
```

### Accéder à un élément : deux façons

```rust
let x = v[0];              // panique si hors bornes
let x = v.get(0);          // Option<&i32> : None si hors bornes
```

`v[i]` est concis mais fait planter le programme. `v.get(i)` t'oblige à traiter l'absence.
Utilise `get` dès que l'index vient de l'extérieur (saisie utilisateur, fichier).

### Itérer : les trois modes (à connaître par cœur)

```rust
for c in &v      { }   // &T      — lecture
for c in &mut v  { }   // &mut T  — modification sur place
for c in v       { }   // T       — CONSOMME le vec, il n'existe plus après
```

Équivalents explicites : `v.iter()`, `v.iter_mut()`, `v.into_iter()`.
L'erreur classique du débutant : écrire `for c in v` puis vouloir réutiliser `v`. Le compilateur
te dira `value moved`. La correction est presque toujours d'ajouter un `&`.

---

## 2. `HashMap<K, V>` et `HashSet<T>`

```rust
use std::collections::HashMap;

let mut decks: HashMap<String, Vec<Card>> = HashMap::new();

decks.insert("rust".to_string(), vec![]);        // renvoie l'ancienne valeur en Option
let cartes = decks.get("rust");                  // Option<&Vec<Card>>
let cartes = decks.get_mut("rust");              // Option<&mut Vec<Card>>
if decks.contains_key("rust") { }
decks.remove("rust");                            // Option<V>

for (nom, cartes) in &decks {                    // ordre NON déterministe !
    println!("{nom}: {}", cartes.len());
}
```

### L'API `entry` — à connaître absolument

Le motif « récupérer ou créer » se fait en une ligne :

```rust
decks.entry("rust".to_string()).or_default().push(carte);
decks.entry(nom).or_insert_with(Vec::new).push(carte);

// compter des occurrences :
*compteur.entry(mot).or_insert(0) += 1;
```

C'est l'idiome à retenir : sans `entry`, tu écrirais un `match get_mut` avec un `insert` dans le
`None`… et le borrow checker te bloquerait (double emprunt de la map).

`HashSet<T>` est le même principe sans valeur : `insert` (renvoie `bool`), `contains`, `remove`,
plus les opérations ensemblistes `union`, `intersection`, `difference`.

> `HashMap` n'a **pas** d'ordre. Si tu veux un ordre stable pour l'affichage ou un fichier de
> sauvegarde, utilise `BTreeMap`/`BTreeSet` (triés par clé) ou trie une copie des clés.

---

## 3. `Option<T>` : l'absence, typée

```rust
enum Option<T> { Some(T), None }
```

### Les méthodes à maîtriser

```rust
opt.is_some() / opt.is_none()

opt.unwrap()                    // panique si None   ⚠️ à éviter hors prototypage
opt.expect("message")           // panique avec TON message  — préférable à unwrap
opt.unwrap_or(0)                // valeur de repli
opt.unwrap_or_else(|| calcul()) // repli calculé paresseusement
opt.unwrap_or_default()         // 0, "", vec![]...

opt.map(|x| x * 2)              // Option<U>  : transforme si Some
opt.and_then(|x| autre(x))      // Option<U>  : enchaîne une fonction qui renvoie Option
opt.filter(|x| *x > 3)          // Option<T>
opt.or(autre_option)
opt.ok_or("pas trouvé")         // Option<T> -> Result<T, E>   ← passerelle très utile
opt.take()                      // sort la valeur et laisse None (sur un &mut)
opt.as_ref()                    // Option<T> -> Option<&T>, pour ne pas déplacer
```

`map` + `unwrap_or` remplace 90 % des `if let` que tu serais tenté d'écrire :

```rust
let titre = carte.map(|c| c.question.clone()).unwrap_or_else(|| "aucune".into());
```

---

## 4. `Result<T, E>` : l'échec, typé

```rust
enum Result<T, E> { Ok(T), Err(E) }
```

Toutes les opérations faillibles de la stdlib renvoient un `Result` : lecture de fichier, parsing,
conversion numérique… Il n'y a **pas d'exceptions** en Rust : une fonction qui peut échouer le dit
dans son type de retour.

`Result` a les mêmes combinateurs qu'`Option` (`map`, `unwrap_or`…) plus :

```rust
res.map_err(|e| MonErreur::Io(e))   // transforme l'erreur
res.ok()                            // Result<T,E> -> Option<T>, en jetant l'erreur
res.is_ok()
```

### L'opérateur `?` — le cœur du chapitre

```rust
fn charger(chemin: &str) -> Result<Config, std::io::Error> {
    let contenu = std::fs::read_to_string(chemin)?;   // si Err → return immédiat avec l'erreur
    let config = analyser(&contenu)?;
    Ok(config)
}
```

`?` signifie : « si c'est `Ok`, donne-moi la valeur ; si c'est `Err`, sors de la fonction en
propageant l'erreur ». C'est le `try/catch` de Rust, en un caractère, visible dans le code.

Deux règles :
- `?` ne s'utilise que dans une fonction qui renvoie `Result` (ou `Option`) ;
- l'erreur est convertie automatiquement via le trait `From` si les types diffèrent — c'est ce qui
  permet d'avoir **un** type d'erreur pour toute l'application.

### `main` peut renvoyer un `Result`

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = charger("config.json")?;
    Ok(())
}
```

`Box<dyn Error>` = « n'importe quelle erreur ». Parfait pour un binaire et pour tes exercices ;
pour une bibliothèque, on définit un enum d'erreurs dédié (voir ci-dessous).

### Ton propre type d'erreur

```rust
#[derive(Debug)]
enum MemoError {
    Io(std::io::Error),
    DeckIntrouvable(String),
    CarteIntrouvable(u32),
    FichierCorrompu { ligne: usize, detail: String },
}

impl std::fmt::Display for MemoError { /* chapitre 6 */ }
impl std::error::Error for MemoError { }

impl From<std::io::Error> for MemoError {          // ← débloque `?` sur les fonctions d'IO
    fn from(e: std::io::Error) -> Self { MemoError::Io(e) }
}
```

C'est le motif complet. Tu l'écriras au chapitre 9 ; à la main d'abord, puis avec la crate
`thiserror` qui le génère pour toi.

### `panic!` : quand est-ce légitime ?

- **Légitime :** un invariant de *ton* code est violé (bug de programmation), un prototype, un test.
- **Illégitime :** une entrée utilisateur invalide, un fichier absent, un réseau coupé.
  Ce sont des situations *attendues* → `Result`.

Repère pour tes revues de code : chaque `unwrap()` dans du code non-test doit être justifiable en
une phrase (« impossible car je viens de vérifier `is_some` »). Sinon, remplace-le.

---

## Exercices

### 4.1 — `ch04-mots-frequents`

Lis un texte (constante multi-ligne dans le code, `r#"..."#` pour une chaîne brute) et affiche les
10 mots les plus fréquents avec leur compte, triés par fréquence décroissante puis alphabétiquement.

**Contraintes :**
- normalise en minuscules, ignore la ponctuation ;
- ignore les mots vides (liste en dur dans un `HashSet`) ;
- utilise l'API `entry` ;
- pour le tri, il faut passer par un `Vec` de paires (une `HashMap` ne se trie pas).

**Indices :** `chars().filter(char::is_alphanumeric)`, `sort_by`, et `cmp` sur des tuples pour un
tri à deux critères. `b.1.cmp(&a.1)` inverse l'ordre.

---

### 4.2 — `ch04-annuaire`

Un petit annuaire interactif en boucle :

```
> ajouter Alice 0612345678
Ajouté.
> chercher Alice
Alice : 0612345678
> chercher Bob
Aucun contact nommé « Bob ».
> lister
Alice : 0612345678
> supprimer Alice
Supprimé.
> quitter
```

**Contraintes :**
- `HashMap<String, String>` en mémoire ;
- **aucun `unwrap()` sur l'entrée utilisateur** : toute commande mal formée affiche une aide, sans
  planter ;
- écris `fn traiter(commande: &str, annuaire: &mut HashMap<String, String>) -> Result<String, String>`
  et laisse `main` se contenter d'afficher `Ok` ou `Err` ;
- valide les numéros : 10 chiffres exactement.

**Indices :** `split_whitespace().collect::<Vec<_>>()` puis un `match` sur la **slice**
(`match parties.as_slice() { ["ajouter", nom, tel] => ..., ... }`) — ce motif est très élégant,
cherche « slice patterns ».

---

### 4.3 — `ch04-notes-eleves`

Structure : `HashMap<String, Vec<f32>>` (élève → notes). Implémente sans jamais paniquer :

```rust
fn ajouter_note(bulletin: &mut Bulletin, eleve: &str, note: f32) -> Result<(), String>  // 0..=20
fn moyenne(bulletin: &Bulletin, eleve: &str) -> Option<f32>       // None si inconnu ou sans note
fn meilleur_eleve(bulletin: &Bulletin) -> Option<(&str, f32)>
fn classement(bulletin: &Bulletin) -> Vec<(&str, f32)>            // trié décroissant
fn eleves_sous_moyenne(bulletin: &Bulletin) -> Vec<&str>
```

**Contraintes :** `type Bulletin = HashMap<String, Vec<f32>>;` (alias de type — pratique).
Aucune de ces fonctions ne doit contenir `unwrap`, `expect` ou `panic!`.

**Piège à comprendre :** pourquoi `meilleur_eleve` renvoie `Option<(&str, f32)>` et pas
`Option<(String, f32)>` ? Que se passerait-il si tu voulais renvoyer `&f32` ? (Ça marche aussi —
essaie, et note ce que ça change pour l'appelant.)

---

### 4.4 — `ch04-calculatrice-rpn`

Une calculatrice en notation polonaise inverse. `"3 4 + 2 *"` → `14`.

```rust
fn evaluer(expression: &str) -> Result<f64, ErreurRpn>
```

avec un enum `ErreurRpn` couvrant au moins : jeton inconnu, pile vide (opérandes manquants),
expression incomplète (plus d'une valeur en fin de pile), division par zéro.

**Contraintes :**
- opérateurs `+ - * / %` et `sqrt` (unaire) ;
- messages d'erreur explicites, affichés dans `main` ;
- `main` renvoie `Result<(), Box<dyn Error>>` et lit les expressions en boucle sur stdin ;
- écris au moins 6 cas de test dans un `#[cfg(test)] mod tests` (formalisé au chapitre 5, mais
  regarde tout de suite la syntaxe : `#[test] fn nom() { assert_eq!(evaluer("3 4 +").unwrap(), 7.0); }`).

**C'est l'exercice de synthèse du chapitre :** `Vec` comme pile, `Result` partout, `?` pour propager,
enum d'erreurs. Prends ton temps.

---

### 4.5 — `ch04-deck` 🎯 **JALON MÉMO 2/9**

Reprends `projet/memo` et ajoute la gestion de collections de cartes.

```rust
struct Library {              // toute la base de l'utilisateur
    cards: Vec<Card>,
    next_id: u32,
}
```

Méthodes attendues :

```rust
fn new() -> Self
fn add(&mut self, deck: &str, question: &str, answer: &str) -> u32     // renvoie l'id attribué
fn get(&self, id: u32) -> Option<&Card>
fn get_mut(&mut self, id: u32) -> Option<&mut Card>
fn remove(&mut self, id: u32) -> Result<Card, MemoError>
fn deck_names(&self) -> Vec<&str>                                      // triés, sans doublon
fn cards_in_deck(&self, deck: &str) -> Vec<&Card>
fn due_cards(&self, deck: Option<&str>, jours: u32) -> Vec<&Card>      // None = tous les paquets
fn count_by_deck(&self) -> BTreeMap<&str, usize>
```

Et un premier `enum MemoError` avec au moins `DeckIntrouvable(String)` et `CarteIntrouvable(u32)`.

**Contraintes :**
- aucune méthode ne panique ;
- `add` refuse une question vide ou un doublon **exact** (même question dans le même paquet) →
  fais-la renvoyer `Result<u32, MemoError>` plutôt que `u32`, et ajoute la variante qui va bien ;
- `deck_names` renvoie des `&str` empruntés à `self` : si ça coince, c'est normal, regarde ce que
  le compilateur propose (et note la question pour le chapitre 8) ;
- dans `main`, construis une bibliothèque de 8 à 10 cartes réparties en 2–3 paquets, et affiche
  un petit rapport (nombre de cartes par paquet, cartes dues).

**Réflexion :** pourquoi `Vec<Card>` plutôt que `HashMap<u32, Card>` ? Écris trois lignes de
commentaire en tête du fichier avec ton raisonnement (ordre stable ? sérialisation ? coût des
recherches par id ?). Il n'y a pas de mauvaise réponse, il y a des arbitrages — mais il faut les
faire consciemment.

---

## ✅ Auto-évaluation

1. Différence entre `v[0]` et `v.get(0)` ?
2. Que fait exactement `?` et où peut-on l'utiliser ?
3. Comment transformer un `Option<T>` en `Result<T, E>` ?
4. Écris l'idiome `entry` pour compter des occurrences.
5. Quand `panic!` est-il légitime ?
6. Quelle est la différence entre `for x in v` et `for x in &v` ?

→ [Chapitre 5 : Modules, tests et crates](../05-modules-tests-et-crates/README.md)
