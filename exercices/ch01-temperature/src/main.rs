fn main() {
    let temperature_c: f64 = 30.0;

    // Temperature
    println!("{} degr", temperature_c);
    println!("{} f", c_vers_f(temperature_c));

    println!("{} f", 86.0);
    print!("{} degr", f_vers_c(86.0));
}

fn c_vers_f(c: f64) -> f64 {
    c * 9.0/5.0 + 32.0
}

fn f_vers_c(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}
