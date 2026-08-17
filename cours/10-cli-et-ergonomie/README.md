# Chapitre 10 — CLI et ergonomie

**Objectif :** transformer ta bibliothèque en un vrai outil en ligne de commande, agréable à
utiliser. Arguments, sous-commandes, couleurs, codes de sortie, et le respect des conventions Unix.

---

## 1. `std::env::args` (pour comprendre)

```rust
let args: Vec<String> = std::env::args().collect();
// args[0] = chemin du programme, args[1..] = les arguments
```

Fais-le une fois pour voir. Puis n'y reviens plus : parser à la main les options courtes, longues,
combinées, les valeurs par défaut et l'aide, c'est 300 lignes de code sans intérêt.

---

## 2. `clap` en mode derive

```bash
cargo add clap --features derive -p memo
```

```rust
use clap::{Parser, Subcommand, ValueEnum};

/// Réviser efficacement grâce à la répétition espacée.
#[derive(Parser)]
#[command(name = "memo", version, about, long_about = None)]
struct Cli {
    /// Chemin du fichier de données
    #[arg(long, global = true)]
    data: Option<PathBuf>,

    /// Affiche plus de détails
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commande,
}

#[derive(Subcommand)]
enum Commande {
    /// Ajoute une carte
    Add {
        #[arg(short, long)]
        deck: String,
        question: String,
        answer: String,
    },
    /// Lance une session de révision
    Review {
        #[arg(short, long)]
        deck: Option<String>,
        #[arg(short, long, default_value_t = 20)]
        limit: usize,
        #[arg(long, value_enum, default_value_t = Algo::Sm2)]
        algo: Algo,
    },
    /// Affiche les statistiques
    Stats,
}

#[derive(Clone, ValueEnum)]
enum Algo { Sm2, Leitner }

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command { ... }
}
```

Ce que clap te donne gratuitement :
- `--help` et `-h` générés **à partir de tes commentaires `///`** ;
- `--version` à partir de `Cargo.toml` ;
- messages d'erreur soignés (« unexpected argument '--dek', did you mean '--deck'? ») ;
- validation des types, valeurs par défaut, valeurs possibles pour les enums ;
- la complétion shell, via la crate `clap_complete`.

Attributs les plus utiles : `#[arg(short, long)]`, `default_value_t`, `value_enum`,
`required = true`, `conflicts_with`, `num_args`, `env = "MEMO_DATA"`.

---

## 3. Les conventions à respecter

Ce sont elles qui font la différence entre « un programme » et « un outil » :

| Convention | Pourquoi |
|------------|----------|
| Résultat sur **stdout**, diagnostics sur **stderr** | permet `memo list > fichier.txt` sans polluer |
| Code de sortie **0** = succès, **≠ 0** = échec | permet `memo review && echo ok` en script |
| `--help` toujours disponible | découvrabilité |
| Pas de couleurs si la sortie n'est pas un terminal | sinon des `\x1b[31m` dans les fichiers |
| Respecter `NO_COLOR` | convention interplateforme (<https://no-color.org>) |
| Lire stdin si `-` est passé en argument | composabilité avec les pipes |
| Ne rien afficher quand il n'y a rien à dire | « no news is good news » |

Codes de sortie :

```rust
fn main() {
    if let Err(e) = executer() {
        eprintln!("Erreur : {e:#}");
        std::process::exit(1);
    }
}
```

⚠️ `std::process::exit` **n'exécute pas les `Drop`** : vide tes tampons avant.

---

## 4. Couleurs et affichage

```bash
cargo add owo-colors -p memo      # ou `colored`, ou `anstyle`
```

```rust
use owo_colors::OwoColorize;
println!("{}", "Correct !".green().bold());
```

Détecter si on est dans un terminal :

```rust
use std::io::IsTerminal;
let couleurs = std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none();
```

`IsTerminal` est dans la bibliothèque standard : pas besoin de crate pour ça.

**Sous Windows :** les terminaux récents (Windows Terminal, PowerShell 7) gèrent les séquences ANSI
nativement. Si tu vois des `[32m` bruts dans une vieille console, c'est ce problème — les crates
citées savent l'activer.

### Aligner du texte

```rust
println!("{:<20} {:>6} {:>8.1}%", nom, total, pourcentage);
println!("{:width$}", texte, width = largeur);
```

Rappel du chapitre 7 : le formatage de Rust compte les **caractères** (`char`), pas la largeur
d'affichage. Un `█`, un emoji ou un idéogramme cassent l'alignement. Pour un tableau propre avec du
texte arbitraire, il te faut `unicode-width`. Pour l'ASCII, `{:<20}` suffit.

---

## 5. Interaction en boucle

Pour la session de révision, tu as besoin de lire des touches. Le minimum :

```rust
print!("Ta réponse (0-3) > ");
io::stdout().flush()?;
let mut ligne = String::new();
io::stdin().read_line(&mut ligne)?;
```

Si tu veux réagir à une touche **sans Entrée**, il faut passer le terminal en mode brut :
`crossterm` (portable, Windows inclus). C'est le bonus du chapitre 12 — commence par la version
« ligne + Entrée », qui a l'avantage d'être testable en redirigeant stdin.

Effacer l'écran entre deux cartes : `print!("\x1B[2J\x1B[1;1H")`, ou `crossterm::terminal::Clear`.

---

## Exercices

### 10.1 — `ch10-wc`

Réimplémente `wc` : compte lignes, mots et octets.

```
$ wc-rs fichier.txt
      42     317    2048 fichier.txt
$ wc-rs -l fichier.txt autre.txt
      42 fichier.txt
      13 autre.txt
      55 total
$ cat fichier.txt | wc-rs
      42     317    2048
```

**Contraintes :**
- options `-l`, `-w`, `-c`, `-m` (caractères) combinables ; aucune option = `-lwc` ;
- plusieurs fichiers → une ligne chacun + un total ;
- aucun fichier, ou `-`, → lit **stdin** ;
- un fichier illisible : message sur stderr, on continue avec les autres, et le code de sortie
  final est 1 ;
- alignement identique au vrai `wc`.

**C'est l'exercice qui apprend les conventions.** Teste avec des pipes et des redirections.

---

### 10.2 — `ch10-grep`

Un chercheur de motif :

```
$ grep-rs --ignore-case -n "todo" src/*.rs
src/main.rs:12: // TODO: gérer le cas vide
```

**Contraintes :**
- options : `-i`, `-n` (numéros de ligne), `-v` (inverser), `-c` (compter seulement),
  `-r` (récursif dans un dossier) ;
- surligne le motif en couleur **seulement si stdout est un terminal** ;
- code de sortie : 0 si au moins une correspondance, 1 sinon, 2 en cas d'erreur (comme le vrai grep) ;
- `-r` sans `walkdir` d'abord (fais-le avec `fs::read_dir` récursif, c'est formateur), puis compare
  avec `cargo add walkdir`.

**Bonus :** `cargo add regex` et ajoute une option `-E` pour les expressions régulières.

---

### 10.3 — `ch10-cli-memo` 🎯 **JALON MÉMO 8/9**

L'interface complète de Mémo. `main.rs` ne fait **que** de la CLI : parsing, affichage, appels à la
bibliothèque.

**Sous-commandes attendues :**

```
memo add --deck <nom> <question> <réponse>     ajoute une carte
memo list [--deck <nom>] [--due] [--all]       liste les cartes
memo review [--deck <nom>] [--limit N] [--algo sm2|leitner]
memo stats [--deck <nom>]                      tableau + barres de progression
memo search <motif>                            recherche plein texte
memo edit <id> [--question Q] [--answer R]     modifie une carte
memo remove <id> [--force]                     supprime (avec confirmation sauf --force)
memo suspend <id> / memo unsuspend <id>
memo import <fichier> --deck <nom>
memo export --deck <nom> <fichier>
memo decks                                     liste les paquets avec leurs compteurs
```

Options globales : `--data <chemin>` (avec repli sur `directories`), `--no-color`, `-v/-vv`.

**Le mode révision — c'est le cœur de l'application :**

```
Paquet « rust » — 7 cartes à réviser (aujourd'hui : 2026-08-17)

[1/7]  Que fait le mot-clé `move` devant une closure ?

       (Entrée pour révéler la réponse, q pour quitter)

       ─────────────────────────────────────────
       Force la closure à prendre possession des variables capturées.

       0 oubliée   1 difficile   2 correcte   3 facile
       > 2
       ✓ Prochaine révision dans 6 jours (2026-08-23).

...

Session terminée en 1 min 12 s
  7 cartes révisées — 6 correctes (86 %)
  1 carte à revoir aujourd'hui
```

**Contraintes de qualité :**
- quitter en cours de session (`q` ou Ctrl+C) **sauvegarde** les cartes déjà révisées ;
- une saisie invalide redemande, ne plante pas et ne saute pas la carte ;
- rien à réviser → message clair et code de sortie 0 (ce n'est pas une erreur) ;
- toutes les erreurs vont sur stderr avec un message utile, jamais un `unwrap` en production ;
- couleurs conditionnées à `is_terminal()` **et** `NO_COLOR` ;
- `memo --help` et `memo review --help` doivent être lisibles et corrects.

**Test manuel de recette (fais-le, sérieusement) :**

```bash
cargo run -p memo -- --data test.json add --deck test "2+2 ?" "4"
cargo run -p memo -- --data test.json list --all
cargo run -p memo -- --data test.json stats
echo 2 | cargo run -p memo -- --data test.json review --deck test
cargo run -p memo -- --data test.json list --due
```

La dernière commande ne doit plus proposer la carte : elle est planifiée à demain.
Si c'est le cas, **ton application marche**. C'est le moment de faire un commit git.

---

## ✅ Auto-évaluation

1. Qu'est-ce qui va sur stdout, qu'est-ce qui va sur stderr ?
2. Que signifie un code de sortie de 0 ? De 1 ?
3. Comment savoir si tu dois colorer ta sortie ?
4. Que génère clap à partir de tes commentaires `///` ?
5. Pourquoi `std::process::exit` est-il risqué avec un `BufWriter` ?
6. Pourquoi lire stdin quand l'argument est `-` ?

→ [Chapitre 11 : Concurrence](../11-concurrence/README.md)
