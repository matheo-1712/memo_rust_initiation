use rand::RngExt;
use std::io::{self};

fn main() {
    gamelogic()
}

fn gamelogic() {
    // -------------------------------------------------------------------------
    // Random number
    let random_number = rand::rng().random_range(1..=100);

    // Le nombre choisi par l'utilisateur
    let mut choice_number = 0;

    // Le tour actuel
    let mut tour = 0;
    println!("Guess the number!");
    // -------------------------------------------------------------------------

    while random_number != choice_number && tour < 6 {

        let mut ligne = String::new();
        io::stdin().read_line(&mut ligne).expect("lecture impossible");

        // On vérifie la validité du nombre
        choice_number = match ligne.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entre un nombre valide");
                continue;
            }
        };

        // On vérifie si le nombre est plus petit ou plus grand
        if choice_number > random_number { println!("Trop grand") }
        else if choice_number < random_number { println!("Trop petit") }
        else { println!("Bravo !") }

        // On incrémente le compteur
        tour += 1;
    }

    if choice_number != random_number {
        println!("Perdu, le chiffre était : {}", random_number)
    }
}