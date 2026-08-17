fn main() {
    let stats: [u32; 10] = [14, 10, 20, 5, 11, 10, 12, 12, 12, 16];
    // On passe une référence (&stats) et on utilise la chaîne de formatage "{}"
    println!("Moyenne : {}", moyenne(&stats));
    println!("Maximum : {}", max(&stats));
    println!("Minimum : {}", min(&stats));
    println!("Etendue : {:?}", etendue(&stats))
}

fn moyenne(stats : &[u32]) -> f64 {
    let mut total : f64 = 0.0;
    for n in 0..stats.len() {
        total += stats[n] as f64;
    }
    // La dernière ligne est return par défaut
    total / stats.len() as f64
}

fn max(stats : &[u32]) -> u32 {
    let mut max : u32 = stats[0];
    for n in 1..stats.len() {
        if stats[n] > max {
            max = stats[n];
        }
    }
    // La dernière ligne est return par défaut
    max
}

fn min(stats : &[u32]) -> u32 {
    let mut min : u32 = stats[0];
    for n in 1..  stats.len() {
        if stats[n] < min {
            min = stats[n];
        }
    }
    // La dernière ligne est return par défaut
    min
}

fn etendue(stats: &[u32]) -> u32 {
    max(stats) - min(stats)
}
