use crate::vec_helper::check_unique_in_1_to_n;
use std::collections::HashSet;

/*
* A struct that models a permutation from some symmetric group S_n,
* i.e. a bijective mapping from {1, ..., n} to {1, ..., n}.
*/
#[derive(Debug, PartialEq)]
pub struct Permutation {
    images: Vec<usize>
}

/*
* Models a cycle in a permutation sigma, 
* i.e. a sequence of numbers i_1, ..., i_r 
* with sigma(i_k) = i_{k+1} and sigma(i_r) = i_1.
*/
#[derive(Debug, PartialEq)]
pub struct Cycle {
    elements: Vec<usize>
}




impl Permutation {
    /*
    * Constructs a new permutation from S_n from the passed number vector of length n.
    * Checks whether the vector defines a bijective mapping on {1, ..., n},
    * if it does not, an appropiate error is returned.
    */
    pub fn new(vec: Vec<usize>) -> Result<Permutation, PermutationError> {
        let n = vec.len();
        
        /*
        * Catching edge case:
        * no meaningful permutation can be created from an empty image vector.
        */
        if n == 0 {
            return Err(PermutationError::EmptyImageVectorError);
        }
        
        /*
        * Iterate over vector and assure that
        * (i)   all numbers occur at most once
        * (ii)  all occuring numbers are in {1, ..., n}
        *
        * (i) ensures injectivity of the mapping described by the vector
        * and since the domain and codomain of the mapping have equal size,
        * this already concludes surjectivity and thus bijectivity.
        */
        if !check_unique_in_1_to_n(&vec, n) {
            return Err(PermutationError::NotBijectiveError);
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

    /*
    * Computes the sign of the permutation sigma 
    * which is the number of inversions in sigma.
    * 
    * An inversion is a tuple (i, j) of numbers in {1, ..., n}
    * where i < j but sigma(i) > sigma(j) 
    */
    pub fn sign(self: &Self) -> i32 {
        let mut inversions = 0;
        let n = self.n();

        for i in 1..(n+1) {
            for j in i..(n+1) {
                if self.eval(i).unwrap() > self.eval(j).unwrap() {
                    inversions += 1;
                }
            }
        }

        if inversions % 2 == 0 { 1 } else { -1 }
    }

    /*
    * Computes the cycle form of some permutation sigma from its table form.
    * So instead of a vector of images, the permutation is represented as a vector of Cycles,
    * where each element from the set {1, ..., n} appears in exactly one cycle.
    */ 
    pub fn to_cycle_form(self: &Self) -> Vec<Cycle> {
        // store set size for readability
        let n = self.n();
        
        // create new empty vector to store the computed cycles
        let mut cycles = Vec::<Cycle>::new();
        
        // create hash set that keeps track of the numbers that already are in a cycle
        let mut nums_in_cycles = HashSet::<usize>::new();

        for i in 1..(n+1) {
            /*
            * If number i is not already contained in a cycle:
            * compute the cycle that starts with i.
            */
            if !nums_in_cycles.contains(&i) {
                /*
                * Create vector that contains the elements of the currently computed cycle.
                * Initially, it only contains i.
                */
                let mut current_cycle_elements = vec![i];
                
                // mark i as being contained in a cycle
                nums_in_cycles.insert(i);

                // compute sigma(i)
                let mut current = self.eval(i).unwrap();

                /*
                * Until i is reached again: 
                * iteratively add the current number to the currently computed cycle,
                * mark it as being contained in a cycle
                * and evaluate sigma for it.
                * Once i is reached again, the cycle is complete.
                */
                while current != i {
                    current_cycle_elements.push(current);

                    nums_in_cycles.insert(current);

                    current = self.eval(current).unwrap();
                }

                // add completed cycle to the vector of cycles.
                cycles.push(Cycle::new(current_cycle_elements, n).unwrap());
            }
        }

        // vector of all cycles of the permutation is returned
        cycles
    }

    /*
    * Computes a string representation of this permutation using its cycle form.
    * I.e. the result looks like "(1 5 4)(2 6)".
    */
    pub fn to_string(self: &Self) -> String {
        // compute cycle form
        let cycle_form = self.to_cycle_form();

        // filter length-1 cycles and retain vector of remaining ones
        let filtered_cycle_form = cycle_form.iter().filter(|c| c.len() > 1);

        // concatenate string representations of remaining cycles
        let mut result = String::new();
        for cycle in filtered_cycle_form {
            result.push_str(&(cycle.to_string()));
        }

        result
    }
}



impl Cycle {
    /*
    * Constructs a new cycle in S_n from the passed vector of non-negative numbers.
    * 
    * If the passed vector does not represent a proper cycle, an error is returned.
    */
    pub fn new(vec: Vec<usize>, n: usize) -> Result<Cycle, PermutationError> {
        if check_unique_in_1_to_n(&vec, n) {
            Ok(
                Cycle {
                    elements: vec
                }
            )
        } else {
            Err(PermutationError::NoValidCycleError)
        }
    }
    
    /*
    * Returns the length of the cycle, 
    * i.e. the number of elements contained in it.
    */
    pub fn len(self: &Self) -> usize {
        self.elements.len()
    }

    /*
    * Computes the string representation of a cycle
    * (which looks like the standard cycle notation,
    * e.g. "(1 4 2 3)")
    */
    pub fn to_string(self: &Self) -> String {
        // filter out special case of empty cycle in the beginning
        if self.elements.len() == 0 {
            return String::from("()");
        }
        
        let mut result = String::from("(");

        for i in 0..self.elements.len() {
            result.push_str(&(self.elements[i].to_string() + " "));
        }

        // trim trailing whitespace and add closing bracket
        result = result[0..result.len()-1].to_string();
        result.push_str(")");

        result
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

/*
* Creates a permutation in S_n that swaps the passed i and j
* and otherwise behaves like the identity.
*/
pub fn transposition(n: usize, i: usize, j: usize) -> Result<Permutation, PermutationError> {
    if i == j {
        return identity(n);
    } else {
        Permutation::new(
            (1..(n+1)).collect::<Vec<usize>>().iter() // iterator over vector (1, ..., n)
                .map( |x| if *x == i {
                            j
                        } else if *x == j {
                            i
                        } else {
                            *x
                        }
                )
                .collect::<Vec<usize>>()
        )
    }
}

/*
* Creates the composition sigma after tau of the two passed permutations sigma and tau. 
*/ 
pub fn compose(sigma: &Permutation, tau: &Permutation) -> Result<Permutation, PermutationError> {
    // compute size of set that sigma operates on
    let n = sigma.n();
    
    /*
    * If domain/range sizes for sigma and tau do not match,
    * the two permutations cannot be composed meaningfully.
    */
    if n != tau.n() {
        return Err(PermutationError::DomainRangeSizeMismatchError);
    }

    Permutation::new(
        (1..(n+1)).collect::<Vec<usize>>().iter() // iterator over (1, ..., n)
            // 1 <= tau(x) <= n is guaranteed since tau is a permutation
            .map( |x| sigma.eval( 
                    // 1 <= x <= n is guaranteed
                    tau.eval(*x).unwrap() 
                ).unwrap() 
            )
            .collect::<Vec<usize>>()
    )
}

/*
* Returns the composition tau after sigma after inverse(tau).
*/
pub fn conjugate(sigma: &Permutation, tau: &Permutation) -> Result<Permutation, PermutationError> {
    compose(
        &(compose(tau, sigma).unwrap()),
        &tau.inverse()
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
    * Returned upon attempt to construct a cycle 
    * that contains numbers outside {1, ..., n}
    * or the same number twice.
    */
    NoValidCycleError,
    /*
    * Occurs when attempting to create a permutation from an empty vector of images.
    */
    EmptyImageVectorError,
    /*
    * Occurs when attempting to compose two permutations from different symmetric groups.
    */
    DomainRangeSizeMismatchError
}