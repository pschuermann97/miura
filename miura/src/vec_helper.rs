/*
* Contains helper functions that operate on integer vectors.
*/

use crate::poly::Modulus;
use std::collections::HashSet;
use std::ops::Mul; // for trait bound for scale vector method


/// Removes the trailing zeros/ multiples of the passed modulus from the passed vector,
/// e.g. vec![2, 3, 0, 0] over modulus None becomes vec![2, 3]
/// and vec![2, 4, 5, 5] over modulus Some(5) becomes vec![2, 4].
pub fn remove_trailing_zeros(vec: &mut Vec<i32>, modulus: Modulus) {
    let mut n = vec.len();
    while 
        vec.len() > 0 && // stop if vector empty (this means the vector models the zero polynomial)
        (modulus == Modulus::None && vec[n-1] == 0 ) // remove trailing zeros for integer polynomial
        || { // all multiples of the modulus are 0 in a remainder class ring
            if let Modulus::Some(x) = modulus {
                vec[n-1] % x == 0
            } else { false }
        }    
    {
        vec.pop();
        n = vec.len(); // coefficients vector was shortened by 1
    }
}


/// Shifts a vector by adding the passed number of zeros at the beginning
/// i.e. vec![1, 1, 426] becomes vec![0, 0, 1, 1, 426] when shifted by 2.
pub fn shift_vector(vec: &Vec<i32>, amt: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];

    for _ in 0..amt {
        result.push(0);
    }

    for x in vec.iter() {
        result.push(*x);
    }

    result
}

/// Scales a vector by the passed factor amt,
/// i.e. vec![3, 2, 1] becomes vec![6, 5, 4] when scaled with 2.
pub fn scale_vector<T>(vec: &Vec<T>, amt: T) -> Vec<T> 
    where
        T: Mul + Copy, 
        Vec<T>: FromIterator<<T as Mul>::Output> 
{
        vec.iter().map(|x| amt * *x).collect::<Vec<T>>()
}

/// Iterate over vector and assure that
/// (i)   all numbers occur at most once
/// (ii)  all occuring numbers are in {1, ..., n}
pub fn check_unique_in_1_to_n(vec: &Vec<usize>, n: usize) -> bool {
    /*
    * For the first requirement, store all numbers seen so far in a hash set
    * and check the set for containment of encountered number.
    */

    let mut occured_numbers: HashSet<usize> = HashSet::new();

    for num in vec.iter() {
        // if you encounter a number for the second time: described mapping is not bijective
        if occured_numbers.contains(num) {
            return false;
        } 
        // otherwise: check whether number is within range and remember it
        else {
            if 1 <= *num && *num <= n {
                occured_numbers.insert(*num);
            } else {
                return false;
            }
        }
    }

    return true;
}

/// Determines whether the passed vector of floating point numbers is the zero vector.
pub fn is_zero_vector(vec: Vec<f32>) -> bool {
    for k in vec {
        if k != 0.0 {
            return false;
        }
    }
    true
}








#[cfg(test)]
mod tests {
    use crate::vec_helper::*;

    #[test]
    fn test_shift_vector() {
        println!("Shift vector by positive number.");

        let vec = vec![426, 99, 71];

        let shifted_vec1 = shift_vector(&vec, 3);

        assert_eq!(shifted_vec1, vec![0, 0, 0, 426, 99, 71]);

        println!("Shift vector by 0.");

        let shifted_vec2 = shift_vector(&vec, 0);

        assert_eq!(shifted_vec2, vec);
    }

    #[test]
    fn test_scale_vector() {
        println!("Scale vector by positive number.");

        let vec = vec![426, 1, 1];
        let scaled_vector1 = scale_vector(&vec, 5);
        assert_eq!(scaled_vector1, vec![2130, 5, 5]);

        println!("Scale vector by negative number.");

        let scaled_vector2 = scale_vector(&vec, -5);
        assert_eq!(scaled_vector2, vec![-2130, -5, -5]);

        println!("Scale vector by 0.");

        let scaled_vector3 = scale_vector(&vec, 0);
        assert_eq!(scaled_vector3, vec![0, 0, 0]);
    }

    #[test]
    fn test_is_zero_vector(){
        println!("Testing zero vector.");

        assert_eq!(is_zero_vector(vec![0.0, 0.0, 0.0, 0.0]), true);

        println!("Testing non-zero vector.");

        assert_eq!(is_zero_vector(vec![426.0, 426.0]), false);
    }
}