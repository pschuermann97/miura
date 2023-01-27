fn main() {
    let separator = "------------------------";

    println!("Euclid: ");
    let a = 23;
    let b = 5002;
    println!("gcd of {} and {} is {}.", a, b, euclid(a, b));
    let a = 100;
    let b = 240;
    println!("gcd of {} and {} is {}.", a, b, euclid(a, b));
    let a = 1402;
    let b = 276;
    println!("gcd of {} and {} is {}.", a, b, euclid(a, b));

    println!("{}", separator);
}

/**
* Computes and returns the greatest common divisor of the input numbers a and b.
*/
fn euclid(a: u32, b:u32) -> u32 {
    if b == 0 {a} else {
        let r = a % b;
        euclid(b, r)
    }
}