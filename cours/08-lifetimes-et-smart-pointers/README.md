# Chapitre 8 — Durées de vie et smart pointers

**Objectif :** comprendre les annotations `'a` qui t'ont fait peur au chapitre 2, et connaître les
outils quand l'ownership simple ne suffit plus : `Box`, `Rc`, `RefCell`, `Cow`.

Chapitre plus théorique. Fais-le sérieusement, mais sache que 80 % du code Rust quotidien n'utilise
que les deux premières sections.

---

## 1. Durées de vie : ce que c'est vraiment

Une durée de vie (*lifetime*) n'est **pas** la durée de vie d'une valeur. C'est une **étiquette**
qui permet au compilateur de relier les références entre elles pour vérifier qu'aucune ne survit à
ce qu'elle pointe.

Elles existent depuis le début — tu ne les écrivais pas parce que le compilateur les devinait.

```rust
fn plus_longue<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

Traduction : « pour une durée `'a`, si tu me donnes deux références valides au moins pendant `'a`,
je te rends une référence valide au moins pendant `'a` ». Le compilateur prendra pour `'a`
l'intersection des deux durées réelles à chaque appel.

Sans l'annotation, le compilateur ne saurait pas si le résultat vient de `a` ou de `b`, donc ne
saurait pas jusqu'à quand il est valide → `E0106: missing lifetime specifier`.

Ça n'a **aucun effet à l'exécution** : c'est purement une vérification statique.

### Les règles d'élision (pourquoi tu n'écris presque jamais `'a`)

1. Chaque paramètre référence reçoit sa propre durée.
2. S'il n'y a **qu'un** paramètre référence, sa durée est donnée au retour.
3. S'il y a `&self`, la durée de `self` est donnée au retour.

C'est la règle 2 qui fait marcher `fn premier_mot(s: &str) -> &str` (exercice 2.2), et la règle 3
qui fait que tes méthodes `fn question(&self) -> &str` compilent sans rien annoter.
Quand tu as deux références en entrée et une en sortie, l'élision abandonne : à toi d'annoter.

### Lifetimes dans les structs

Une struct qui contient une référence doit dire de quoi elle dépend :

```rust
struct Extrait<'a> {
    source: &'a str,
    debut: usize,
}

impl<'a> Extrait<'a> {
    fn texte(&self) -> &str { &self.source[self.debut..] }
}
```

Signification : « une `Extrait` ne peut pas survivre au texte qu'elle référence ». Le compilateur
l'imposera.

**Conseil pour la vraie vie :** tant que tu débutes, préfère des structs **possédantes**
(`String`, `Vec<T>`) plutôt qu'empruntantes. Les structs à lifetime sont utiles pour les parseurs
zéro-copie et l'optimisation ; elles compliquent beaucoup l'API. Le bon réflexe n'est pas
« éviter le clone à tout prix », c'est « posséder par défaut, emprunter quand c'est mesurément utile ».

### `'static`

`&'static T` = référence valide pendant toute la durée du programme. Les littéraux de chaîne sont
`&'static str` (ils sont dans le binaire). `T: 'static` en borne veut dire « ce type ne contient
aucune référence empruntée » — c'est la contrainte que tu croiseras au chapitre 11 avec les threads.

---

## 2. `Box<T>` — allouer sur le tas

```rust
let b = Box::new(42);          // le 42 est sur le tas, b est le propriétaire
println!("{}", *b);            // déréférencement
```

Trois usages réels :

**a) Les types récursifs.** Sans indirection, la taille serait infinie :

```rust
enum Expr {
    Nombre(f64),
    Addition(Box<Expr>, Box<Expr>),      // sans Box : "recursive type has infinite size"
    Negation(Box<Expr>),
}
```

**b) Les objets-traits.** `Box<dyn Scheduler>` (chapitre 6) : la taille n'est pas connue à la
compilation, le `Box` la rend connue (c'est un pointeur).

**c) Déplacer une grosse valeur sans la copier.** Rare, mesure d'abord.

---

## 3. `Rc<T>` — propriété partagée

Quand une valeur doit avoir **plusieurs** propriétaires (un graphe, un arbre avec liens vers le
parent, un cache partagé) :

```rust
use std::rc::Rc;

let a = Rc::new(String::from("partagé"));
let b = Rc::clone(&a);          // +1 au compteur, PAS de copie des données
println!("{}", Rc::strong_count(&a));   // 2
```

Comptage de références : la valeur est libérée quand le dernier `Rc` disparaît.
`Rc::clone` est bon marché (incrémentation d'un entier) — on l'écrit `Rc::clone(&a)` plutôt que
`a.clone()` justement pour signaler au lecteur que ce n'est pas une copie profonde.

⚠️ `Rc` est **mono-thread**. Sa version atomique pour le multi-thread est `Arc` (chapitre 11).
⚠️ Les cycles de `Rc` fuient (la mémoire n'est jamais libérée). Solution : `Weak<T>` pour les liens
« retour » (parent → enfant en `Rc`, enfant → parent en `Weak`).

`Rc<T>` ne donne qu'un accès **en lecture**. Pour muter, il faut le combiner avec…

---

## 4. `RefCell<T>` — mutabilité intérieure

`RefCell` déplace la vérification d'emprunt de la **compilation** vers l'**exécution** :

```rust
use std::cell::RefCell;

let cellule = RefCell::new(vec![1, 2, 3]);
cellule.borrow_mut().push(4);            // emprunt mutable vérifié à l'exécution
println!("{:?}", cellule.borrow());
```

Les règles sont les mêmes (un seul `borrow_mut` à la fois), mais leur violation provoque un
**panic** au lieu d'une erreur de compilation. Tu échanges de la sécurité statique contre de la
souplesse : à n'utiliser que quand tu ne peux vraiment pas faire autrement.

La combinaison classique : `Rc<RefCell<T>>` = « plusieurs propriétaires, tous peuvent modifier ».
C'est le plus proche d'un objet partagé façon Python/Java.

**Mise en garde :** si tu atteins `Rc<RefCell<T>>` dès le début d'un projet, c'est souvent le signe
que tu plaques une architecture orientée objet sur Rust. Souvent, une meilleure conception existe :
un `Vec` central avec des index/ids (c'est le choix de Mémo : `Library` possède les cartes, tout le
monde manipule des `u32`), ou un passage de `&mut` là où il faut. C'est la conception « arena »,
très courante en Rust.

---

## 5. `Cow<'a, T>` — emprunter, cloner seulement si nécessaire

```rust
use std::borrow::Cow;

fn nettoyer(entree: &str) -> Cow<'_, str> {
    if entree.contains('\t') {
        Cow::Owned(entree.replace('\t', " "))   // on a dû allouer
    } else {
        Cow::Borrowed(entree)                    // rien à faire, zéro allocation
    }
}
```

*Clone-on-write.* Élégant pour les fonctions de normalisation appelées très souvent, où le cas
fréquent est « rien à changer ».

---

## 6. Récapitulatif : quel outil quand ?

| Besoin | Outil |
|--------|-------|
| Un seul propriétaire, taille connue | la valeur elle-même |
| Lire sans posséder | `&T` |
| Modifier sans posséder | `&mut T` |
| Taille inconnue / récursion / objet-trait | `Box<T>` |
| Plusieurs propriétaires (mono-thread) | `Rc<T>` |
| Plusieurs propriétaires + mutation | `Rc<RefCell<T>>` |
| Idem, multi-thread | `Arc<Mutex<T>>` (ch. 11) |
| Éviter une allocation dans le cas fréquent | `Cow<'a, T>` |
| Graphe avec cycles | `Rc` + `Weak` — ou des index dans un `Vec` |

---

## Exercices

### 8.1 — `ch08-lifetimes-lecture`

Pour chacune de ces signatures, dis **sans compiler** si elle compile, et pourquoi :

```rust
fn a(x: &str) -> &str
fn b(x: &str, y: &str) -> &str
fn c(x: &str, y: &str) -> String
fn d<'a>(x: &'a str, y: &str) -> &'a str
fn e(&self) -> &str                       // dans un impl
fn f<'a>(x: &'a str) -> &'static str
struct G { texte: &str }
struct H<'a> { texte: &'a str }
```

Puis vérifie avec `cargo check` en les écrivant toutes (corps minimal). Note tes erreurs de
prédiction : elles te disent exactement quelle règle d'élision tu n'as pas intégrée.

---

### 8.2 — `ch08-analyseur`

Écris un analyseur de texte **zéro-copie** :

```rust
struct Analyseur<'a> { texte: &'a str }

impl<'a> Analyseur<'a> {
    fn nouveau(texte: &'a str) -> Self
    fn phrases(&self) -> Vec<&'a str>
    fn mots_uniques(&self) -> HashSet<&'a str>
    fn contexte(&self, mot: &str, marge: usize) -> Vec<&'a str>   // extraits autour des occurrences
    fn plus_longue_phrase(&self) -> Option<&'a str>
}
```

**Contraintes :** aucun `String`, aucun `to_string()`, aucun `clone()` dans tout le fichier.
Toutes les sorties sont des vues sur le texte d'origine.

**Puis, l'exercice dans l'exercice :** essaie d'écrire dans `main` une fonction qui crée un
`Analyseur` sur une `String` locale et le renvoie. Lis l'erreur, comprends-la, et écris en
commentaire pourquoi c'est précisément le bug que Rust vient de t'éviter.

---

### 8.3 — `ch08-arbre-expression`

Un évaluateur d'expressions arithmétiques avec un enum récursif :

```rust
enum Expr {
    Nombre(f64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

fn evaluer(e: &Expr, vars: &HashMap<String, f64>) -> Result<f64, ErreurEval>
fn afficher(e: &Expr) -> String        // avec parenthèses minimales — c'est le morceau difficile
fn simplifier(e: Expr) -> Expr         // x*1 -> x, x+0 -> x, 0*x -> 0, constantes pliées
fn variables(e: &Expr) -> HashSet<&str>
```

**Contraintes :** construis les expressions à la main dans `main` (pas besoin de parser).
`evaluer` renvoie une erreur explicite pour une variable inconnue et pour une division par zéro.

**Bonus costaud :** réutilise ton tokenizer de l'exercice 3.3 et écris un parseur en descente
récursive pour lire `"2 * (x + 3)"`. C'est un rite de passage classique, et Rust y est très bon.

---

### 8.4 — `ch08-vues` 🎯 **JALON MÉMO 6/9**

Deux améliorations de `projet/memo` qui exercent le chapitre.

**a) Une session de révision empruntée.**

```rust
pub struct ReviewSession<'a> {
    library: &'a mut Library,
    queue: Vec<u32>,            // des IDs, pas des références — réfléchis à pourquoi
    position: usize,
    correct: usize,
    started_at: std::time::Instant,
}

impl<'a> ReviewSession<'a> {
    pub fn new(library: &'a mut Library, deck: Option<&str>, limite: usize) -> Self
    pub fn current(&self) -> Option<&Card>
    pub fn answer(&mut self, grade: Grade, scheduler: &dyn Scheduler) -> Result<(), MemoError>
    pub fn progress(&self) -> (usize, usize)          // (fait, total)
    pub fn summary(&self) -> SessionSummary           // consomme ou emprunte, à toi de choisir
}
```

**Le point pédagogique :** `queue` contient des `u32` et non des `&Card`. Essaie la version avec
`Vec<&'a Card>` et constate que `answer` (qui a besoin de `&mut library`) devient impossible :
tu aurais un emprunt partagé vivant pendant un emprunt exclusif. **Écris cette tentative, lis
l'erreur, puis reviens aux ids.** C'est le motif « index plutôt que référence » de la section 4,
et c'est une leçon d'architecture Rust que tu réutiliseras toute ta vie.

**b) Normalisation avec `Cow`.**

```rust
pub fn normalize_question(q: &str) -> Cow<'_, str>
```

Elle réduit les espaces multiples et retire les espaces en fin de ligne — mais ne doit **rien
allouer** quand la question est déjà propre (le cas courant). Utilise-la dans `Library::add` pour la
détection de doublons.

**Tests attendus :** que la variante `Borrowed` est bien renvoyée pour une entrée propre
(`matches!(resultat, Cow::Borrowed(_))`), et `Owned` sinon.

---

## ✅ Auto-évaluation

1. Une annotation `'a` change-t-elle quelque chose à l'exécution ?
2. Énonce les trois règles d'élision.
3. Quand une struct a-t-elle besoin d'un paramètre de durée de vie ?
4. `Box` vs `Rc` vs `RefCell` : une phrase chacun.
5. Que se passe-t-il si tu appelles `borrow_mut()` deux fois de suite sur le même `RefCell` ?
6. Pourquoi stocker des identifiants plutôt que des références dans une structure de session ?

→ [Chapitre 9 : Fichiers et sérialisation](../09-fichiers-et-serialisation/README.md)
