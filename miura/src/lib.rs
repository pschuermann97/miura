// module declarations
mod poly;

/**
* Computes and returns the greatest common divisor of the input numbers a and b.
*/
fn euclid(a: u32, b:u32) -> u32 {
    if b == 0 {a} else {
        let r = a % b;
        euclid(b, r)
    }
}

/**
* Uses the quicksort algorithm to sort the passed array of positive integers.
*/
fn quicksort(a: &Vec<u32>) -> Vec<u32> {
    // empty list and list with only one element are trivially sorted
    if a.len() <= 1 {
        return a.to_vec();
    }

    // isolate first element in the array (pivot) from the rest of the vector
    let pivot = a[0];
    let rest = &a[1..];

    /*
    * Split up list into two lists:
    * those of the elements that are smaller and those that are greater/equal than the pivot.
    *
    * Note: I feel like this would be more elegant with closures to compute left and right.
    */
    let mut left = Vec::new();
    let mut right = Vec::new();
    for &elem in (*rest).iter() { // iterator contains references
        if elem < pivot {
            left.push(elem);
        }
        else {
            right.push(elem);
        }
    }

    /*
    * Recursively solve the subproblems and concatenate the results.
    */
    let mut result:Vec<u32> = Vec::new();
    result.append(&mut quicksort(&left));
    result.push(pivot);
    result.append(&mut quicksort(&right));
    result
}

/*
* Module for unit-tests of this file.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly::*;

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

    #[test]
    fn test_quicksort() {
        let vector1 = vec![3, 2, 4, 1, 5];

        let vector1_sorted = quicksort(&vector1);

        assert_eq!(vector1_sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn degree_test() {
        println!("Testing integer polynomial 1 + X + X^2 + X^3.");

        let poly1 = IntPoly::new(
            &mut vec![1, 1, 1, 1],
            Modulus::None
        );
        assert_eq!(poly1.deg(), 3);

        println!("Testing integer polynomial 426 + X + 0X^2.");

        let poly2 = IntPoly::new(
            &mut vec![426, 1, 0],
            Modulus::None
        );
        assert_eq!(poly2.deg(), 1);
    }
}
