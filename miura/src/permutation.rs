use std::collections::HashSet; // for ensuring bijectivity

/*
* A struct that models a permutation from some symmetric group S_n,
* i.e. a bijective mapping from {1, ..., n} to {1, ..., n}.
*/
#[derive(Debug, PartialEq)]
pub struct Permutation {
    images: Vec<usize>
}

impl Permutation {
    /*
    * Constructs a new permutation from S_n from the passed number vector of length n.
    * Checks whether the vector defines a bijective mapping on {1, ..., n},
    * if it does not, an appropiate error is returned.
    */
    pub fn new(vec: Vec<usize>) -> Result<Permutation, PermutationError> {
        /*
        * Catching edge case:
        * no meaningful permutation can be created from an empty image vector.
        */
        if vec.len() == 0 {
            return Err(PermutationError::EmptyImageVectorError);
        }
        
        /*
        * Iterate over vector and assure that
        * (i)   all numbers occur at most once
        * (ii)  all occuring numbers are in {1, ..., n}
        *
        * For the first requirement, store all numbers seen so far in a hash set
        * and check the set for containment of encountered number.
        *
        * (i) ensures injectivity of the mapping described by the vector
        * and since the domain and codomain of the mapping have equal size,
        * this already concludes surjectivity and thus bijectivity.
        */

        let mut occured_numbers: HashSet<usize> = HashSet::new();

        for num in vec.iter() {
            // if you encounter a number for the second time: described mapping is not bijective
            if occured_numbers.contains(num) {
                return Err(PermutationError::NotBijectiveError);
            } 
            // otherwise: check whether number is within range and remember it
            else {
                if 1 <= *num && *num <= vec.len() {
                    occured_numbers.insert(*num);
                } else {
                    return Err(PermutationError::ImageOutOfRangeError);
                }
            }
        }

        // now we know that the passed vector models a bijective mapping from {1, ..., n} to {1, ..., n}

        Ok(
            Permutation{
                images: vec
            }
        )
    }

    /*
    * Returns the size of the set n that this permutation operates on.
    */
    pub fn n(self: &Self) -> usize {
        self.images.len()
    }

    /*
    * Evaluates the permutation for the passed number.
    * If the number is not in the set that the permutation operates on,
    * an error is returned.
    */
    pub fn eval(self: &Self, i: usize) -> Result<usize, PermutationError> {
        if i > self.n() || i <= 0 {
            Err(PermutationError::ArgOutOfRangeError)
        } else {
            /*
            * The domain of a permutation s is {1, ..., n}
            * but the indexing of vector elements starts at 0.
            */
            Ok(self.images[i - 1])
        }
    }

    /*
    * Computes the inverse of the permutation.
    */
    pub fn inverse(self: &Self) -> Permutation {
        let mut inverse_images = Vec::<usize>::new();
        let n = self.images.len();
        
        // compute image under inverse separately for every i in {1, ..., n}
        for i in 1..(n+1) {
            /*
            * Find element in {1, ..., n} that maps to i 
            * by looking for index j in the image vector that stores i.
            */
            for j in 0..n {
                if self.images[j] == i {
                    /*
                    * Permutations in S_n map from and to the set {1, ..., n}
                    * but the indices of a vector of length n range from 0 to n-1.
                    */
                    inverse_images.push(j+1);

                    break; // can stop looking once we found the right j
                }
            }
        }

        Permutation::new(
            inverse_images
        ).unwrap() // if a mapping is a permutation then so is its inverse -> always Ok-variant
    }
}

/*
* Returns the identity function on the set {1, ..., n} 
* which is the neutral element of the symmetric group S_n.
*/
pub fn identity(n: usize) -> Result<Permutation, PermutationError> {
    // catch special case that n is 0: S_0 does not exist
    if n==0 {
        return Err(PermutationError::EmptyImageVectorError);
    }
    
    Permutation::new(
        (1..(n+1)).collect::<Vec<usize>>()
    )
}

#[derive(Debug, PartialEq)]
pub enum PermutationError {
    /*
    * Returned upon attempt to evaluate the permutation for a value outside {1, ..., n}
    * for the respective n of the permutation.
    */
    ArgOutOfRangeError,
    /*
    * Returned upon attempt to create a permutation that returns a value outside {1, ..., n}
    * for the respective n of the permutation.
    */
    ImageOutOfRangeError,
    /*
    * Returned upon attempt to create a permutation that is not bijective.
    */
    NotBijectiveError,
    /*
    * Occurs when attempting to create a permutation from an empty vector of images.
    */
    EmptyImageVectorError

}