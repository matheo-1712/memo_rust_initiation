fn main() {
    println!("{}", fizzbuzz(1));
}

fn fizzbuzz(mut n: u32) -> String {
    let mut counter: String = "".to_string();
    let separator = "\n";

    while n <= 100 {
        counter += &multiple_format(n);
        counter += separator;
        n += 1;
    }

    // On renvoie 'counter'
    counter
}

fn multiple_format(n: u32) -> String {
    // On teste 3 et 5 si les 2 sont présent c'est un multiple de 15
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}