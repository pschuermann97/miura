/*
* A struct that models a permutation from some symmetric group S_n,
* i.e. a bijective mapping from {1, ..., n} to {1, ..., n}.
*/
struct Permutation {
    images: Vec<i32>
}

impl Permutation {
    /*
    * Returns the size of the set n that this permutation operates on.
    */
    pub fn n(self: &Self) -> i32 {
        self.images.len()
    }

    /*
    * Evaluates the permutation for the passed number.
    * If the number is not in the set that the permutation operates on,
    * an error is returned.
    */
    pub fn eval(self: &Self, i: i32) -> Result<i32, PermutationError> {
        if i > self.n() {
            Err(PermutationError::OutOfRangeError)
        } else {
            /*
            * The domain of a permutation s is {1, ..., n}
            * but the indexing of vector elements starts at 0.
            */
            self.images[i - 1]
        }
    }
}

enum PermutationError {
    /*
    * Returned upon attempt to evaluate the permutation for a value outside {1, ..., n}
    * for the respective n of the permutation.
    */
    OutOfRangeError
}