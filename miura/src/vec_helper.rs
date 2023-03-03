/*
* Contains helper functions that operate on integer vectors.
*/

use crate::poly::Modulus;


/*
* Removes the trailing zeros/ multiples of the passed modulus from the passed vector,
* e.g. vec![2, 3, 0, 0] over modulus None becomes vec![2, 3]
* and vec![2, 4, 5, 5] over modulus Some(5) becomes vec![2, 4].
*/
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

/*
* Shifts a vector by adding the passed number of zeros at the beginning
* i.e. vec![1, 1, 426] becomes vec![0, 0, 1, 1, 426] when shifted by 2.
*/
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

/*
* Scales a vector by the passed factor amt,
* i.e. vec![3, 2, 1] becomes vec![6, 5, 4] when scaled with 2.
*/
pub fn scale_vector(vec: &Vec<i32>, amt: i32) -> Vec<i32> {
    vec.iter().map(|x| amt * x).collect::<Vec<i32>>()
}