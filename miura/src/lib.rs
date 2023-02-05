/**
* Computes and returns the greatest common divisor of the input numbers a and b.
*/
fn euclid(a: u32, b:u32) -> u32 {
    if b == 0 {a} else {
        let r = a % b;
        euclid(b, r)
    }
}

/*
* Module for unit-tests of this file.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclid() {
        println!("test two coprime values");
        assert_eq!(euclid(13, 4), 1);

        println!("test two non-coprime values");
        assert_eq!(euclid(84, 144), 12);

        println!("test two large values");
        assert_eq!(euclid(426426, 5184), 6);

        println!("test two values where a<b");
        assert_eq!(euclid(134, 426), 2);

        println!("test with a=0");
        assert_eq!(euclid(0, 71), 71);

        println!("test with b=0");
        assert_eq!(euclid(23, 0), 23);
    }
}
