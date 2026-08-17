# Chapitre 11 — Concurrence

**Objectif :** utiliser plusieurs threads sans risque. C'est le domaine où Rust brille le plus fort :
la « fearless concurrency » n'est pas un slogan marketing, c'est une conséquence directe de
l'ownership que tu as appris au chapitre 2.

---

## 1. Pourquoi Rust change la donne

Une *data race* survient quand deux threads accèdent à la même mémoire, qu'au moins un écrit, et
qu'il n'y a pas de synchronisation. C'est la classe de bugs la plus pénible qui soit :
non déterministe, invisible en test, catastrophique en production.

Rappelle-toi la règle du chapitre 2 : **soit plusieurs lecteurs, soit un seul écrivain**.
C'est exactement la condition qui empêche les data races. Rust ne l'a pas ajoutée pour la
concurrence : elle y était déjà, et elle rend les data races **impossibles à compiler**.

Deux traits marqueurs, appliqués automatiquement par le compilateur :

- **`Send`** : le type peut être *transféré* vers un autre thread.
- **`Sync`** : le type peut être *partagé* par référence entre threads (`&T` est `Send`).

Presque tout est `Send + Sync`. Les exceptions notables : `Rc<T>` (compteur non atomique → utilise
`Arc`), `RefCell<T>` (vérification non atomique → utilise `Mutex` ou `RwLock`). Si tu essaies de
les envoyer dans un thread, le compilateur refuse, avec le nom du trait manquant dans le message.

---

## 2. Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("dans le thread");
    42                                  // valeur de retour
});

let resultat = handle.join().unwrap();  // attend la fin ; Err si le thread a paniqué
```

`join()` renvoie un `Result` parce que le thread peut avoir paniqué — un panic ne fait pas tomber
tout le programme, seulement son thread.

Capturer des données : il faut `move`, car le compilateur ne sait pas si le thread survivra à la
portée courante.

```rust
let donnees = vec![1, 2, 3];
thread::spawn(move || println!("{donnees:?}"));   // donnees est déplacée dans le thread
```

C'est la borne `'static` du chapitre 8 en action : la closure ne peut contenir aucune référence
empruntée à la pile courante.

### Threads scopés (souvent la meilleure réponse)

```rust
let mut donnees = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| println!("{:?}", &donnees));      // emprunt PARTAGÉ autorisé
    s.spawn(|| println!("longueur : {}", donnees.len()));
});                                               // tous les threads sont joints ici
// donnees est réutilisable
```

`thread::scope` garantit que les threads se terminent avant la fin du bloc, donc les emprunts sont
sûrs — pas besoin de `Arc`, pas de `move`, pas de clone. Depuis Rust 1.63, c'est souvent la solution
la plus simple pour paralléliser un calcul sur des données locales.

---

## 3. Partager de l'état

### `Arc<T>` — le `Rc` atomique

```rust
use std::sync::Arc;

let partage = Arc::new(gros_vecteur);
for _ in 0..4 {
    let copie = Arc::clone(&partage);
    thread::spawn(move || { /* lecture seule de `copie` */ });
}
```

### `Mutex<T>` — exclusion mutuelle

```rust
use std::sync::{Arc, Mutex};

let compteur = Arc::new(Mutex::new(0));

let c = Arc::clone(&compteur);
thread::spawn(move || {
    let mut verrou = c.lock().unwrap();   // bloque jusqu'à obtenir le verrou
    *verrou += 1;
});                                        // le verrou est libéré au drop
```

Détail remarquable : en Rust, le `Mutex` **contient** la donnée. Il est donc *impossible* d'accéder
à la donnée sans prendre le verrou — l'erreur la plus commune dans les autres langages n'existe pas.

`lock()` renvoie un `Result` : il est `Err` si un autre thread a paniqué en tenant le verrou
(mutex « empoisonné »).

⚠️ **Portée du verrou.** `let v = m.lock().unwrap();` garde le verrou jusqu'à la fin du bloc.
Si tu fais une opération longue (IO, calcul) pendant, tu sérialises tout. Encadre par `{ }` ou
utilise `drop(v)` pour libérer tôt.

`RwLock<T>` est la variante « plusieurs lecteurs OU un écrivain » : plus efficace quand les lectures
dominent largement.

### Canaux (message passing)

Le style recommandé quand c'est possible : *« ne communiquez pas en partageant la mémoire ;
partagez la mémoire en communiquant »*.

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

for i in 0..4 {
    let tx = tx.clone();                       // multi producteurs
    thread::spawn(move || tx.send(travail(i)).unwrap());
}
drop(tx);                                       // ⚠️ sinon rx n'atteint jamais la fin

for resultat in rx {                            // itère jusqu'à fermeture de tous les tx
    println!("{resultat:?}");
}
```

Le `drop(tx)` oublié est **le** bug classique du canal : le programme se bloque à la fin parce qu'un
émetteur reste vivant.

---

## 4. `rayon` — le parallélisme facile

```bash
cargo add rayon
```

```rust
use rayon::prelude::*;

let somme: u64 = donnees.par_iter().map(couteux).sum();
resultats.par_sort();
```

Tu remplaces `iter()` par `par_iter()` et c'est parallélisé, avec un vol de travail équilibré.
Ça n'est possible que parce que le système de types garantit l'absence de data races : la
bibliothèque peut se permettre une API aussi simple.

**Mesure toujours.** Sous ~10 000 éléments avec un travail léger par élément, le coût de
coordination dépasse le gain. `cargo add --dev criterion` pour benchmarker sérieusement, ou
`Instant::now()` pour une mesure grossière.

---

## 5. Un mot sur l'async

`async`/`await` est **autre chose** que les threads : c'est de la concurrence pour du code
« en attente » (réseau, IO), pas du parallélisme de calcul. Ça demande un exécuteur
(`tokio`, `smol`) et ça a ses propres règles (`Pin`, `Send` sur les futures…).

**Ce n'est pas le sujet de ce cours** et Mémo n'en a aucun besoin : ton goulot d'étranglement est
l'utilisateur qui tape au clavier. Retiens la règle de choix :

- beaucoup d'attente d'IO (serveur web, client HTTP) → **async** ;
- beaucoup de calcul à répartir sur les cœurs → **threads / rayon** ;
- ni l'un ni l'autre → **rien**, garde ton code séquentiel.

Le meilleur code concurrent est celui qu'on n'écrit pas.

---

## Exercices

### 11.1 — `ch11-mandelbrot`

Calcule une image de l'ensemble de Mandelbrot en ASCII (80×40, puis 800×400 en PGM si tu veux voir
quelque chose de joli).

**Marche à suivre imposée :**
1. version séquentielle, chronométrée ;
2. version avec `thread::scope`, en découpant l'image en N bandes horizontales (une par cœur —
   `std::thread::available_parallelism()`) ;
3. version avec `rayon` (`par_chunks_mut` sur le tampon de pixels) ;
4. tableau comparatif des trois temps en `--release`, et calcul de l'accélération.

**Question à laquelle répondre en commentaire :** pourquoi le découpage en bandes contiguës donne-t-il
un déséquilibre de charge, et comment `rayon` s'en sort-il mieux ?

---

### 11.2 — `ch11-compteur-mots-parallele`

Compte les mots de tous les fichiers `.md` de ce dépôt de cours (récursif), en parallèle.

**Contraintes :**
- version A : `Arc<Mutex<HashMap<String, usize>>>` — un seul verrou global ;
- version B : chaque thread construit sa **propre** `HashMap`, et on fusionne à la fin
  (via un canal ou en collectant les `JoinHandle`) ;
- chronomètre les deux et explique l'écart.

**C'est la leçon la plus importante du chapitre :** la version B est presque toujours plus rapide,
parce que le verrou global sérialise tout le travail utile. « Diviser puis fusionner » bat
« partager et verrouiller » dans la grande majorité des cas.

**Bonus :** version C avec `rayon` et `fold`/`reduce`. Compare la lisibilité.

---

### 11.3 — `ch11-import-parallele` 🎯 **JALON MÉMO 9/9**

Dernier jalon avant l'assemblage final.

**a) Import parallèle.**

```rust
memo import decks/*.txt --deck-from-filename
```

Importe plusieurs fichiers en une commande, chacun lu et analysé dans un thread, avec un rapport
final :

```
✓ rust.txt        42 cartes ajoutées, 3 doublons ignorés
✓ anglais.txt     18 cartes ajoutées
✗ cassé.txt       ligne 7 : séparateur « | » manquant
Total : 60 cartes ajoutées dans 2 paquets, 1 fichier en erreur.
```

**Contraintes :**
- le **parsing** est parallèle, l'**insertion** dans la `Library` reste séquentielle (elle a besoin
  de `&mut self` et doit attribuer des ids uniques et déterministes) — réfléchis à cette séparation
  avant de coder, c'est tout l'exercice ;
- l'ordre du rapport final est **déterministe** (trié par nom de fichier), même si les threads
  finissent dans le désordre ;
- une erreur dans un fichier n'annule pas les autres ;
- code de sortie 1 s'il y a eu au moins une erreur ;
- teste avec 5 fichiers dont 2 volontairement cassés.

**Question de conception :** aurais-tu pu utiliser `Arc<Mutex<Library>>` et tout faire en parallèle ?
Oui. Écris en commentaire pourquoi c'est une mauvaise idée ici (indice : quel serait l'ordre des
identifiants attribués d'une exécution à l'autre ? tes tests seraient-ils reproductibles ?).

**b) Sauvegarde en arrière-plan (optionnel, plus délicat).**

Pendant une longue session de révision, sauvegarder toutes les 10 cartes sans bloquer l'utilisateur :
un thread dédié qui reçoit des instantanés par canal et les écrit atomiquement.

**Contraintes :** l'utilisateur ne doit jamais attendre une écriture disque, et le programme doit
attendre la fin de la dernière écriture avant de sortir (`join`). Vérifie qu'un Ctrl+C au mauvais
moment ne corrompt rien — tu as déjà l'écriture atomique du chapitre 9.

---

## ✅ Auto-évaluation

1. Qu'est-ce qu'une data race, et pourquoi ne peut-elle pas compiler en Rust ?
2. `Send` vs `Sync` ?
3. Pourquoi `Rc` ne peut-il pas traverser un thread ? Par quoi le remplacer ?
4. Quel avantage a `thread::scope` sur `Arc` + `move` ?
5. Pourquoi faut-il `drop(tx)` avant d'itérer sur `rx` ?
6. Pourquoi « diviser puis fusionner » bat-il souvent un `Mutex` global ?
7. Threads ou async : comment choisis-tu ?

→ [Chapitre 12 : Projet final](../12-projet-final-memo/README.md)
