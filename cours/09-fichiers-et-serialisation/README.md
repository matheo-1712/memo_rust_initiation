# Chapitre 9 — Fichiers, dates et sérialisation

**Objectif :** rendre Mémo persistant. Lire et écrire des fichiers proprement, sérialiser en JSON
avec serde, manipuler des dates, et gérer les erreurs d'IO comme un adulte.

---

## 1. Lire et écrire : les raccourcis

```rust
use std::fs;

let contenu: String = fs::read_to_string("cartes.json")?;
let octets: Vec<u8> = fs::read("image.png")?;

fs::write("cartes.json", contenu)?;         // écrase, crée si besoin
fs::create_dir_all("data/decks")?;          // comme mkdir -p, ok si déjà là
fs::remove_file("vieux.json")?;
fs::rename("tmp.json", "cartes.json")?;     // atomique sur le même volume
let existe = std::path::Path::new("x.json").exists();
```

Pour 99 % des cas de ce cours, `read_to_string` et `write` suffisent.

### Quand le fichier est gros : les tampons

```rust
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

let fichier = File::open("gros.txt")?;
for ligne in BufReader::new(fichier).lines() {
    let ligne = ligne?;              // chaque ligne est un Result !
    // ...
}

let mut sortie = BufWriter::new(File::create("sortie.txt")?);
writeln!(sortie, "{ligne}")?;
sortie.flush()?;                     // ⚠️ ne l'oublie pas
```

Sans `BufWriter`, chaque `write!` fait un appel système : sur 100 000 lignes, c'est 50× plus lent.
Et un `BufWriter` non vidé perd silencieusement ses dernières données (il vide au `drop`, mais si
le programme se termine par `process::exit`, c'est perdu).

### Chemins : `Path` et `PathBuf`

Même duo qu'au chapitre 2 : `PathBuf` possède, `&Path` emprunte.

```rust
use std::path::{Path, PathBuf};

let mut chemin = PathBuf::from("data");
chemin.push("decks");
chemin.push("rust.json");                     // data/decks/rust.json (séparateur adapté à l'OS)
chemin.set_extension("bak");

let dossier = chemin.parent();                // Option<&Path>
let nom = chemin.file_name();                 // Option<&OsStr>
let ext = chemin.extension();
```

**Ne construis jamais un chemin par concaténation de `String`** avec `/` ou `\` : `PathBuf::push`
fait ça correctement sur Windows comme sur Linux. Ton projet tourne sous Windows ; ce détail
compte.

Où stocker les données d'une application ? Pas à côté de l'exécutable. La convention :

```bash
cargo add directories
```

`ProjectDirs::from("fr", "toi", "memo")` te donne le bon dossier sur chaque OS
(`%APPDATA%\toi\memo` sous Windows).

### L'écriture atomique (à faire pour Mémo)

Si le programme est interrompu au milieu d'un `fs::write`, tu perds toute la base. Le motif sûr :

1. écrire dans `cartes.json.tmp` ;
2. `flush` + fermer ;
3. `fs::rename("cartes.json.tmp", "cartes.json")` — atomique au niveau du système de fichiers.

Trois lignes qui transforment « corruption possible » en « impossible ». C'est ce que fait tout
logiciel sérieux qui écrit un fichier de données.

---

## 2. serde : sérialisation

```bash
cargo add serde --features derive -p memo
cargo add serde_json -p memo
```

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: u32,
    pub question: String,
    #[serde(default)]                        // absent dans le JSON → valeur par défaut
    pub tags: Vec<String>,
    #[serde(rename = "createdAt")]           // nom différent dans le JSON
    pub created_at: String,
    #[serde(skip)]                           // jamais sérialisé
    pub cache: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
```

```rust
let json = serde_json::to_string(&library)?;
let json = serde_json::to_string_pretty(&library)?;    // lisible par un humain
let library: Library = serde_json::from_str(&json)?;
```

Les enums se sérialisent aussi, et bien :

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum CardState {
    New,
    Learning { step: u8 },
    Scheduled { interval_days: u32, ease: f32 },
}
```

donne `{"type": "learning", "step": 2}`. Explore aussi `#[serde(untagged)]`, `#[serde(flatten)]`.
La doc de référence est excellente : <https://serde.rs/attributes.html>.

**Le versionnage du format.** Ton fichier de sauvegarde va évoluer. Prévois dès maintenant un champ
`version: u32` dans la structure racine, et refuse poliment de charger une version inconnue.
Ça te coûte 3 lignes aujourd'hui et t'évite un fichier illisible dans six mois.

---

## 3. Les dates avec `chrono`

```bash
cargo add chrono --features serde -p memo
```

```rust
use chrono::{DateTime, Duration, NaiveDate, Utc};

let maintenant: DateTime<Utc> = Utc::now();
let aujourdhui: NaiveDate = Utc::now().date_naive();
let dans_six_jours = aujourdhui + Duration::days(6);
let ecart = (echeance - aujourdhui).num_days();      // i64
let texte = aujourdhui.format("%Y-%m-%d").to_string();
let parsee = NaiveDate::parse_from_str("2026-08-17", "%Y-%m-%d")?;
```

Pour Mémo, `NaiveDate` (une date sans heure ni fuseau) est le bon type : une carte est due « le
17 août », pas « le 17 août à 14 h 32 UTC ». Moins de fuseaux horaires = moins de bugs.

Avec la feature `serde`, `NaiveDate` se sérialise en `"2026-08-17"`.

⚠️ Vérifie l'API dans `cargo doc -p chrono --open` : certaines constructions (`Duration::days`,
`chrono::Days`, les méthodes `checked_add_*`) ont évolué selon les versions, et les tutoriels en
ligne sont souvent périmés. C'est le même exercice de lecture de doc qu'au chapitre 1.

### Tester du code qui dépend de « maintenant »

Un test qui appelle `Utc::now()` est un test qui échouera un jour à minuit. Le remède, universel :
**injecte l'horloge**.

```rust
pub trait Clock { fn today(&self) -> NaiveDate; }

pub struct SystemClock;
pub struct FixedClock(pub NaiveDate);      // pour les tests
```

Toutes tes fonctions métier prennent `today: NaiveDate` en paramètre plutôt que de l'aller chercher.
Seule la couche CLI appelle la vraie horloge. C'est de l'injection de dépendance, en cinq lignes,
et ça rend testable l'algorithme entier de répétition espacée.

---

## 4. Erreurs d'IO propres

C'est le bon moment pour finir `MemoError`. À la main d'abord (tu l'as vu au chapitre 4), puis :

```bash
cargo add thiserror -p memo
```

```rust
#[derive(Debug, thiserror::Error)]
pub enum MemoError {
    #[error("erreur d'entrée/sortie : {0}")]
    Io(#[from] std::io::Error),                  // génère From<io::Error> automatiquement

    #[error("fichier de données illisible : {0}")]
    Json(#[from] serde_json::Error),

    #[error("le paquet « {0} » n'existe pas")]
    DeckNotFound(String),

    #[error("aucune carte avec l'identifiant {0}")]
    CardNotFound(u32),

    #[error("format de fichier version {trouvee}, cette version de memo lit jusqu'à {max}")]
    UnsupportedVersion { trouvee: u32, max: u32 },
}
```

`thiserror` génère `Display`, `Error` et les `From`. Pour un **binaire**, la crate sœur `anyhow`
(avec `anyhow::Result` et `.context("en chargeant la base")`) est encore plus pratique.
La convention de l'écosystème : `thiserror` dans une bibliothèque (erreurs typées, l'appelant
décide), `anyhow` dans une application (erreurs opaques, on affiche et on sort).

Mémo étant les deux (`lib.rs` + `main.rs`), tu peux faire les deux : `thiserror` dans la lib,
`anyhow` dans le `main`.

Un mot sur les messages : un bon message d'erreur dit **ce qui a échoué, sur quoi, et quoi faire**.
Compare « Erreur : No such file or directory (os error 2) » avec
« Impossible de lire C:\Users\toi\AppData\Roaming\memo\cards.json : le fichier n'existe pas.
Lance `memo init` pour créer une base vide. »

---

## Exercices

### 9.1 — `ch09-journal`

Un journal de bord en ligne de commande, stocké en JSON Lines (un objet JSON par ligne) :

```
> ajouter Première entrée du jour
Entrée #1 enregistrée.
> lister
[2026-08-17 09:12] #1 Première entrée du jour
> chercher jour
1 résultat.
```

**Contraintes :**
- fichier `journal.jsonl`, une entrée = une ligne ;
- l'ajout doit être **incrémental** (ouvrir en mode append, pas réécrire tout le fichier) —
  cherche `OpenOptions::new().append(true).create(true)` ;
- une ligne corrompue au milieu du fichier ne doit pas empêcher de lire les autres : signale-la sur
  stderr et continue ;
- horodatage avec chrono.

**Question :** pourquoi JSON Lines plutôt qu'un gros tableau JSON, pour un journal ? (Écris la
réponse en tête de fichier.)

---

### 9.2 — `ch09-config`

Une struct `Config` avec 6 champs de types variés (`String`, `u32`, `bool`, `Option<String>`,
un enum, un `Vec<String>`). Écris :

```rust
fn charger(chemin: &Path) -> Result<Config, ConfigError>   // fichier absent → Config::default()
fn sauver(config: &Config, chemin: &Path) -> Result<(), ConfigError>   // ÉCRITURE ATOMIQUE
fn fusionner(fichier: Config, cli: ConfigPartielle) -> Config          // la CLI a priorité
```

**Contraintes :**
- `#[serde(default)]` pour que l'ajout d'un champ ne casse pas les anciens fichiers ;
- un test qui écrit dans un dossier temporaire (`cargo add --dev tempfile`) : jamais de test qui
  pollue le vrai disque de l'utilisateur ;
- un test qui vérifie l'aller-retour : `charger(sauver(c)) == c` (propriété de *round-trip*) ;
- un test qui vérifie qu'un JSON invalide produit une erreur, pas un panic.

---

### 9.3 — `ch09-persistance` 🎯 **JALON MÉMO 7/9**

Mémo devient utilisable pour de vrai.

**a) Dates réelles.** Remplace le `jours_ecoules: u32` bidon du chapitre 3 par de vraies dates :

```rust
enum CardState {
    New,
    Learning { step: u8, due: NaiveDate },
    Scheduled { due: NaiveDate, interval_days: u32, ease: f32 },
    Suspended,
}
```

Ajoute à `Card` : `created_at: NaiveDate`, `reviews: u32`, `lapses: u32`
(nombre d'oublis — utile pour les stats et pour repérer les cartes mal formulées).

Adapte `Scheduler::next` pour qu'il prenne `today: NaiveDate` en paramètre. **Aucun appel à
`Utc::now()` en dehors de `main.rs`** — c'est la règle de la section 3, et tes tests en dépendent.

**b) Sérialisation.**

```rust
#[derive(Serialize, Deserialize)]
pub struct Library {
    version: u32,
    cards: Vec<Card>,
    next_id: u32,
}

impl Library {
    pub fn load(path: &Path) -> Result<Self, MemoError>   // fichier absent → bibliothèque vide
    pub fn save(&self, path: &Path) -> Result<(), MemoError>   // ATOMIQUE
    pub fn default_path() -> PathBuf                       // via `directories`
    pub fn export_deck(&self, deck: &str, path: &Path) -> Result<usize, MemoError>   // texte
    pub fn import(&mut self, path: &Path, deck: &str) -> Result<usize, MemoError>
}
```

Le format d'import/export texte, simple et lisible à la main :

```
# Un commentaire
Que fait `move` ? | Force la closure à prendre possession des captures.
Qu'est-ce qu'une slice ? | Une vue empruntée sur une portion contiguë.
```

L'import doit : ignorer les lignes vides et les commentaires, signaler le **numéro de ligne** des
lignes mal formées, ne pas réimporter un doublon, et renvoyer le nombre de cartes ajoutées.

**c) `MemoError` complet** avec `thiserror`, dont la gestion de version de fichier.

**Tests exigés :**
- round-trip : `save` puis `load` redonne une bibliothèque identique (dérive `PartialEq`) ;
- charger un fichier inexistant → bibliothèque vide, **pas** une erreur ;
- charger un JSON corrompu → `Err(MemoError::Json(_))`, pas un panic ;
- charger un fichier avec `version: 999` → `Err(UnsupportedVersion { .. })` ;
- import d'un fichier avec 2 lignes valides, 1 commentaire, 1 ligne cassée → 2 cartes ajoutées et
  une erreur signalée mentionnant la ligne 4 ;
- tous les tests d'IO utilisent `tempfile`.

**Test de robustesse à faire une fois, à la main :** lance une sauvegarde et tue le processus
(Ctrl+C) pendant l'écriture, plusieurs fois. Avec l'écriture atomique, le fichier doit **toujours**
être lisible. C'est le genre de propriété qu'on n'obtient pas par hasard.

---

## ✅ Auto-évaluation

1. Pourquoi `BufWriter` plutôt que d'écrire directement dans un `File` ?
2. Décris l'écriture atomique en trois étapes et dis ce qu'elle protège.
3. Pourquoi ne jamais construire un chemin en concaténant des `String` ?
4. À quoi sert `#[serde(default)]` et quel problème d'évolution ça règle ?
5. Pourquoi passer `today: NaiveDate` en paramètre plutôt qu'appeler `Utc::now()` dans la fonction ?
6. `thiserror` ou `anyhow` : lequel dans une bibliothèque, lequel dans un binaire, et pourquoi ?

→ [Chapitre 10 : CLI et ergonomie](../10-cli-et-ergonomie/README.md)
