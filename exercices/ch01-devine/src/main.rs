use std::io::{self, Write};
use rand::{random, RngExt};

fn main() {
    gamelogic()
}

fn gamelogic() {
    // -------------------------------------------------------------------------
    // Random number
    let random_number = rand::rng().random_range(1..=100);
    // DEBUG
    println!("{}", random_number);

    // Le nombre choisi par l'utilisateur
    let mut choice_number = 0;
    println!("Guess the number!");
    // -------------------------------------------------------------------------

    while random_number != choice_number {
        let mut ligne = String::new();
        io::stdin().read_line(&mut ligne).expect("lecture impossible");
        choice_number = ligne.trim().parse().expect("pas un nombre");

        // On vérifie si le nombre est plus petit ou plus grand
        if choice_number > random_number {println!("Trop grand")}
        else { println!("Trop petit") }
    }
}