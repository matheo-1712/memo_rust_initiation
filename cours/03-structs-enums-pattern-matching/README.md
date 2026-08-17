# Chapitre 3 — Structs, enums et pattern matching

**Objectif :** modéliser un domaine. C'est là que Rust devient plaisant : son système de types
te permet de rendre les états invalides **impossibles à représenter**.

À la fin de ce chapitre, tu écris le premier morceau de Mémo.

---

## 1. Structs

```rust
struct Carte {
    id: u32,
    question: String,
    reponse: String,
    facilite: f32,
}

let c = Carte {
    id: 1,
    question: String::from("Capitale du Japon ?"),
    reponse: String::from("Tokyo"),
    facilite: 2.5,
};

println!("{}", c.question);
```

Variantes utiles :

```rust
struct Point(f64, f64);      // tuple struct : champs anonymes, accès .0 / .1
struct Metre(f64);           // newtype : un Metre n'est PAS un f64, le compilateur les distingue
struct Marqueur;             // unit struct : pas de données
```

Le **newtype** est un motif que tu utiliseras beaucoup : `struct CardId(u32)` empêche de passer un
`DeckId` là où on attend un `CardId`. Zéro coût à l'exécution, un bug de moins.

### Mutabilité

Elle porte sur **la variable entière**, pas sur les champs :

```rust
let mut c = Carte { ... };
c.facilite = 2.6;      // ok car `c` est mut
```

Il n'existe pas de « champ mutable » individuel. Si un seul champ doit changer alors que la struct
est partagée en lecture, c'est le chapitre 8 (`RefCell`).

### Sucre syntaxique

```rust
fn nouvelle(question: String, reponse: String) -> Carte {
    Carte { question, reponse, id: 0, facilite: 2.5 }   // raccourci : champ = variable de même nom
}

let modifiee = Carte { facilite: 3.0, ..c };   // reste des champs pris dans `c` (attention : move !)
```

---

## 2. `impl` : méthodes et fonctions associées

```rust
impl Carte {
    // fonction associée (pas de self) : s'appelle Carte::nouvelle(...)
    fn nouvelle(question: &str, reponse: &str) -> Self {
        Self { id: 0, question: question.to_string(), reponse: reponse.to_string(), facilite: 2.5 }
    }

    // méthode en lecture : s'appelle c.resume()
    fn resume(&self) -> &str { &self.question }

    // méthode qui modifie
    fn renforcer(&mut self, delta: f32) { self.facilite += delta; }

    // méthode qui consomme (rare, mais utile pour les builders)
    fn en_texte(self) -> String { self.question }
}
```

Les trois formes de `self` sont exactement les trois modes du chapitre 2 :
`&self` (emprunt partagé), `&mut self` (emprunt exclusif), `self` (move).
Choisis toujours **le plus faible qui suffit**.

`Self` (majuscule) = le type en cours. `self` (minuscule) = l'instance.

Tu peux avoir plusieurs blocs `impl` pour un même type. Ça sert à regrouper par thème.

---

## 3. Enums : le vrai super-pouvoir

Un enum Rust n'est pas une liste de constantes : c'est un **type somme**, chaque variante pouvant
porter ses propres données.

```rust
enum Reponse {
    Oubliee,
    Difficile,
    Correcte,
    Facile,
}

enum Evenement {
    Ajout { carte_id: u32 },              // variante struct
    Revision(u32, Reponse),               // variante tuple
    Suppression(u32),
    Reinitialisation,                     // variante unit
}
```

Une valeur d'`Evenement` est **exactement une** de ces quatre formes, et le compilateur t'oblige
à traiter les quatre cas. C'est ce qui remplace, en Rust : les hiérarchies de classes, les champs
« utilisés seulement si type == X », les codes d'erreur entiers, et les `null`.

### Les deux enums de la bibliothèque standard

```rust
enum Option<T> { Some(T), None }          // il y a une valeur, ou pas
enum Result<T, E> { Ok(T), Err(E) }       // ça a marché, ou ça a échoué avec une raison
```

**Rust n'a pas de `null`.** L'absence est un `Option`, et le compilateur t'empêche d'oublier de la
traiter. C'est un bug entier de l'informatique (le « milliard de dollars d'erreur ») supprimé par
construction. Chapitre 4 pour les manipuler couramment.

---

## 4. Pattern matching

```rust
let points = match reponse {
    Reponse::Oubliee   => 0,
    Reponse::Difficile => 1,
    Reponse::Correcte  => 2,
    Reponse::Facile    => 3,
};
```

`match` est **exhaustif** : si tu ajoutes une variante `Reponse::Sautee` plus tard, tous les `match`
du projet cesseront de compiler jusqu'à ce que tu décides quoi en faire. C'est une fonctionnalité,
pas une nuisance : le compilateur te fait la liste des choses à mettre à jour.

### Extraire les données

```rust
match evenement {
    Evenement::Ajout { carte_id } => println!("ajout #{carte_id}"),
    Evenement::Revision(id, Reponse::Oubliee) => println!("#{id} oubliée"),   // motif imbriqué
    Evenement::Revision(id, _) => println!("#{id} révisée"),
    Evenement::Suppression(id) if id > 100 => println!("suppression tardive"), // garde
    _ => println!("autre"),                                                     // fourre-tout
}
```

Ce que tu peux mettre dans un motif : littéraux, intervalles (`1..=9`), alternatives (`'a' | 'e'`),
déstructurations de struct/tuple/enum, `_` (ignorer), `..` (ignorer le reste), liaisons `nom @ motif`.

⚠️ L'ordre compte : le premier motif qui correspond gagne. Un `_` placé trop tôt rend le reste
inaccessible (clippy te préviendra).

### `if let` et `let else`

Quand un seul cas t'intéresse :

```rust
if let Some(valeur) = mon_option {
    println!("{valeur}");
}

while let Some(sommet) = pile.pop() { }

let Some(valeur) = mon_option else {
    return;                 // le `else` doit diverger : return / break / panic!
};
// ici, `valeur` est disponible dans la suite de la fonction
```

`let ... else` est excellent pour aplatir les fonctions : tu traites les cas d'échec en haut et le
chemin nominal reste à plat, sans imbrication.

---

## 5. Les dérivations

```rust
#[derive(Debug, Clone, PartialEq)]
struct Carte { ... }
```

`derive` demande au compilateur d'implémenter automatiquement un trait. Les indispensables :

| Trait | Donne accès à | Note |
|-------|---------------|------|
| `Debug` | `{:?}` et `{:#?}` | mets-le **partout**, toujours |
| `Clone` | `.clone()` | duplication explicite |
| `Copy` | copie implicite | uniquement si tous les champs sont `Copy` |
| `PartialEq` | `==`, `!=` | |
| `Eq`, `Hash` | clés de `HashMap`/`HashSet` | `Eq` impossible pour les flottants |
| `PartialOrd`, `Ord` | `<`, `sort()` | ordre = ordre de déclaration des champs/variantes |
| `Default` | `Type::default()` | 0, `false`, `""`, `None`… |

Règle de départ : `#[derive(Debug, Clone)]` sur toutes tes structs, et tu ajoutes le reste au besoin.

---

## 6. Modéliser proprement : « rendre l'invalide impossible »

Compare :

```rust
// ❌ modélisation faible
struct Carte {
    statut: String,          // "nouvelle" ? "Nouvelle" ? "nouvelel" ?
    date_revision: Option<String>,
    intervalle_jours: i32,   // et si c'est -3 ?
}

// ✅ modélisation forte
enum Statut {
    Nouvelle,
    EnApprentissage { etape: u8 },
    Planifiee { intervalle_jours: u32, facilite: f32 },
    Suspendue,
}
```

Dans la seconde, il est **impossible** d'avoir un intervalle négatif ou un statut mal orthographié,
et impossible d'oublier de gérer un cas. Ce réflexe — « quel type rend ce bug inexprimable ? » —
est le cœur du métier en Rust. Garde-le en tête pour le jalon ci-dessous.

---

## Exercices

### 3.1 — `ch03-formes`

Un enum `Forme` avec les variantes `Cercle`, `Rectangle`, `Triangle` (choisis les données de
chacune), et un `impl Forme` avec :

```rust
fn aire(&self) -> f64
fn perimetre(&self) -> f64
fn nom(&self) -> &str
fn est_valide(&self) -> bool     // dimensions > 0, et inégalité triangulaire pour le triangle
```

Dans `main`, mets un `Vec<Forme>`, affiche un tableau aligné (nom, aire, périmètre à 2 décimales)
et la forme de plus grande aire.

**Indices :** `f64::consts::PI`, `sqrt()`, formule de Héron pour l'aire d'un triangle à partir de
ses trois côtés. Pour le max sur des `f64`, `max()` n'existe pas sur les itérateurs de flottants —
cherche pourquoi (indice : `NaN`) et contourne avec une boucle pour l'instant.

---

### 3.2 — `ch03-etat-commande`

Modélise le cycle de vie d'une commande en ligne avec un enum :
`Panier`, `Payee { moyen: MoyenPaiement }`, `Expediee { transporteur: String, suivi: String }`,
`Livree { date: String }`, `Annulee { raison: String }`.

Écris `fn transition(etat: Etat, evenement: Evenement) -> Result<Etat, String>` qui applique une
machine à états : payer un panier → `Payee`, expédier une commande payée → `Expediee`, etc.
Toute transition illégale renvoie `Err` avec un message explicite
(« impossible d'expédier une commande annulée »).

**Contraintes :**
- la fonction **consomme** l'état et en renvoie un nouveau (pas de `&mut`) ;
- aucun `_ =>` fourre-tout dans le `match` principal : liste explicitement les couples valides,
  puis un cas final qui construit le message d'erreur ;
- dans `main`, déroule un scénario nominal et deux scénarios fautifs.

**Ce que tu apprends ici :** une machine à états typée. C'est exactement le motif que tu réutiliseras
pour le statut d'une carte de révision.

---

### 3.3 — `ch03-json-tokens` (pattern matching intensif)

Écris un **tokenizer** minimal : une fonction `fn tokeniser(entree: &str) -> Vec<Token>` où

```rust
enum Token {
    AccoladeOuvrante, AccoladeFermante,
    CrochetOuvrant, CrochetFermant,
    DeuxPoints, Virgule,
    Texte(String),
    Nombre(f64),
    Booleen(bool),
    Nul,
    Inconnu(char),
}
```

Elle transforme `{"a": [1, true]}` en la suite de tokens correspondante.

**Contraintes :**
- ignore les espaces ;
- une chaîne est délimitée par `"` (pas besoin de gérer les échappements) ;
- un nombre peut être négatif et décimal ;
- affiche le résultat avec `{:#?}`.

**Indices :** itère avec `chars().peekable()` — `peek()` te laisse regarder le caractère suivant sans
le consommer, indispensable pour savoir où s'arrête un nombre. C'est le premier exercice où tu
sentiras la puissance du `match` sur des caractères.

**Bonus :** ajoute une variante `Token::Erreur(String)` et signale une chaîne non fermée.

---

### 3.4 — `ch03-carte` 🎯 **JALON MÉMO 1/9**

On commence l'application. Crée la crate du projet :

```bash
cargo new projet/memo
```

Dans `src/main.rs` (on découpera en modules au chapitre 5), définis le cœur du domaine :

**`struct Card`** avec au minimum :
- un identifiant,
- une question et une réponse,
- le nom du paquet auquel elle appartient,
- son état de révision.

**`enum CardState`** — c'est ici que tu appliques la section 6. Une carte est :
- soit **jamais vue**,
- soit **en cours d'apprentissage** (elle repasse plusieurs fois dans la même session),
- soit **planifiée** avec un intervalle en jours et un facteur de facilité,
- soit **suspendue**.

**`enum Grade`** : la note que l'utilisateur donne à sa propre réponse — oubliée, difficile,
correcte, facile.

**Méthodes à implémenter (`impl Card`) :**

```rust
fn new(id: u32, deck: &str, question: &str, answer: &str) -> Self
fn is_due(&self, jours_ecoules: u32) -> bool   // version naïve : pas encore de vraies dates
fn preview(&self) -> String                     // "#12 [rust] Que fait `move` ?..." tronqué à 60 car.
fn suspend(&mut self)
fn resume(&mut self)
```

**Contraintes :**
- `#[derive(Debug, Clone, PartialEq)]` sur les trois types ;
- **aucun champ `String` pour représenter un état** : si tu écris `statut: String`, tu as raté
  l'exercice ;
- `preview` tronque proprement — attention, tronquer une `String` UTF-8 par octets peut paniquer
  (revois le chapitre 2, section 5) ;
- dans `main`, crée trois cartes en dur, affiche-les, suspends-en une, et affiche `{:#?}`.

**Critère de réussite :** ajoute une cinquième variante à `CardState` (par exemple `Enterree`) et
constate que le compilateur te liste **exactement** tous les endroits à mettre à jour. Puis retire-la.
C'est la sensation que tu dois retenir de ce chapitre.

> ⚠️ Ne code pas encore l'algorithme de répétition espacée. Il arrive au chapitre 6, quand tu auras
> les traits pour le rendre remplaçable. Pour l'instant : que des données et des accesseurs.

---

## ✅ Auto-évaluation

1. Quelle est la différence entre `Self` et `self` ?
2. Quand utiliser `&self`, `&mut self`, `self` ?
3. Pourquoi un enum Rust est-il plus puissant qu'un enum Java/C ?
4. Que se passe-t-il si un `match` ne couvre pas tous les cas ?
5. Qu'est-ce qu'un newtype et à quoi ça sert ?
6. Cite trois traits que tu dérives quasi systématiquement.

→ [Chapitre 4 : Collections et gestion d'erreurs](../04-collections-et-erreurs/README.md)
