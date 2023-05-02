//! An API to work with mathematical permutations
//! i.e. bijective mappings from {1, ..., n} to {1, ..., n}.



use crate::vec_helper::check_unique_in_1_to_n;
use std::collections::HashSet;

/// A struct that models a permutation from some symmetric group S_n,
/// i.e. a bijective mapping from {1, ..., n} to {1, ..., n}.
#[derive(Debug, PartialEq)]
pub struct Permutation {
    images: Vec<usize>
}

/// Models a cycle in a permutation sigma, 
/// i.e. a sequence of numbers i_1, ..., i_r 
/// with sigma(i_k) = i_{k+1} and sigma(i_r) = i_1.
#[derive(Debug, PartialEq)]
pub struct Cycle {
    elements: Vec<usize>
}




impl Permutation {
    /// Constructs a new permutation from S_n from the passed number vector of length n.
    /// Checks whether the vector defines a bijective mapping on {1, ..., n},
    /// if it does not, an appropiate error is returned.
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

    /// Returns the size of the set {1, ..., n} that this permutation operates on.
    pub fn n(self: &Self) -> usize {
        self.images.len()
    }

    /// Evaluates the permutation for the passed number.
    /// If the number is not in the set that the permutation operates on,
    /// an error is returned.
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

    /// Computes the inverse of the permutation.
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

    /// Computes the sign of the permutation sigma 
    /// which is the number of inversions in sigma.
    /// 
    /// An inversion is a tuple (i, j) of numbers in {1, ..., n}
    /// where i < j but sigma(i) > sigma(j) 
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

    /// Computes the cycle form of some permutation sigma from its table form.
    /// So instead of a vector of images, the permutation is represented as a vector of Cycles,
    /// where each element from the set {1, ..., n} appears in exactly one cycle.
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

    /// Computes a string representation of this permutation using its cycle form.
    /// I.e. the result looks like "(1 5 4)(2 6)".
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
    /// Constructs a new cycle in S_n from the passed vector of non-negative numbers.
    /// 
    /// If the passed vector does not represent a proper cycle, an error is returned.
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
    
    /// Returns the length of the cycle, 
    /// i.e. the number of elements contained in it.
    pub fn len(self: &Self) -> usize {
        self.elements.len()
    }

    /// Computes the string representation of a cycle
    /// (which looks like the standard cycle notation,
    /// e.g. "(1 4 2 3)")
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




/// Returns the identity function on the set {1, ..., n} 
/// which is the neutral element of the symmetric group S_n.
pub fn identity(n: usize) -> Result<Permutation, PermutationError> {
    // catch special case that n is 0: S_0 does not exist
    if n==0 {
        return Err(PermutationError::EmptyImageVectorError);
    }
    
    Permutation::new(
        (1..(n+1)).collect::<Vec<usize>>()
    )
}

/// Creates a permutation in S_n that swaps the passed i and j
/// and otherwise behaves like the identity.
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

/// Creates the composition sigma after tau of the two passed permutations sigma and tau. 
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

/// Returns the composition tau after sigma after inverse(tau).
pub fn conjugate(sigma: &Permutation, tau: &Permutation) -> Result<Permutation, PermutationError> {
    compose(
        &(compose(tau, sigma).unwrap()),
        &tau.inverse()
    )
}

/// A type that models all kinds of errors
/// that can occur when working with permutations.
#[derive(Debug, PartialEq)]
pub enum PermutationError {
    /// Returned upon attempt to evaluate the permutation for a value outside {1, ..., n}
    /// for the respective n of the permutation.
    ArgOutOfRangeError,
    /// Returned upon attempt to create a permutation that returns a value outside {1, ..., n}
    /// for the respective n of the permutation.
    ImageOutOfRangeError,
    /// Returned upon attempt to create a permutation that is not bijective.
    NotBijectiveError,
    /// Returned upon attempt to construct a cycle 
    /// that contains numbers outside {1, ..., n}
    /// or the same number twice.
    NoValidCycleError,
    /// Occurs when attempting to create a permutation from an empty vector of images.
    EmptyImageVectorError,
    /// Occurs when attempting to compose two permutations from different symmetric groups.
    DomainRangeSizeMismatchError
}










#[cfg(test)]
mod tests {
    use crate::permutation::*;

    #[test]
    fn permutation_constructor_test() {
        println!("Creating a proper permutation.");

        let image_vec1 = vec![5, 3, 2, 1, 4];
        assert!(
            Permutation::new(image_vec1).is_ok()
        );

        println!("Attempt to create a permutation that has an out-of-range image.");

        let image_vec2 = vec![2, 6, 4, 1, 3];
        assert_eq!(
            Permutation::new(image_vec2),
            Err(PermutationError::NotBijectiveError)
        );

        println!("Attempt to create a permutation that is not bijective.");

        let image_vec3 = vec![1, 2, 3, 4, 1];
        assert_eq!(
            Permutation::new(image_vec3),
            Err(PermutationError::NotBijectiveError)
        );

        println!("Attempt to create a permutation for an empty image vector.");

        assert_eq!(
            Permutation::new(vec![]),
            Err(PermutationError::EmptyImageVectorError)
        );
    }

    #[test]
    /*
    * Tests whether the size of the set that the permutation operates on
    * can be correctly computed.
    */
    fn n_test() {
        println!("Creating S_1 permutation.");

        let n1 = Permutation::new(vec![1]).unwrap().n();
        assert_eq!(n1, 1);

        println!("Creating S_5 permutation.");

        let n2 = Permutation::new(vec![5, 3, 2, 1, 4]).unwrap().n();
        assert_eq!(n2, 5);
    }

    #[test]
    fn eval_test() {
        println!("Creating an S_5 permutation.");

        let sigma = Permutation::new(
            vec![3, 4, 2, 1, 5]
        ).unwrap();

        println!("Checking evaluation for 3");

        assert_eq!(sigma.eval(3), Ok(2));

        println!("Asserting that evaluation for 426 fails.");

        assert_eq!(sigma.eval(426), Err(PermutationError::ArgOutOfRangeError));
    }

    #[test]
    fn identity_test() {
        println!("Creating identity function in S_3.");
        
        assert_eq!(
            identity(3),
            Permutation::new(
                vec![1, 2, 3]
            )
        );

        println!("Asserting that suitable error is returned when attempting to create identity for S_0.");
    }

    #[test]
    fn inverse_test() {
        println!("Computing the inverse of a non-identity S_5 permutation.");

        let sigma = Permutation::new(
            vec![3, 4, 1, 5, 2]
        ).unwrap();

        assert_eq!(
            sigma.inverse(),
            Permutation::new(
                vec![3, 5, 1, 2, 4]
            ).unwrap()
        );

        println!("Computing the inverse of the identity S_5 permutation.");

        assert_eq!(
            identity(5).unwrap().inverse(),
            identity(5).unwrap()
        );
    }

    #[test]
    fn transposition_test() {
        println!("Creating transposition in S_5.");

        assert_eq!(
            transposition(5, 2, 3),
            Permutation::new(
                vec![1, 3, 2, 4, 5]
            )
        );

        println!("Asserting that order of i and j parameters does not matter.");

        assert_eq!(
            transposition(5, 2, 3),
            transposition(5, 3, 2)
        );

        println!("Creating identity on S_6 via transposition function.");

        assert_eq!(
            transposition(6, 1, 1),
            identity(6)
        );

        println!("Asserting that we cannot swap values that are outside the domain (more precisely, transposition(5, 2, 426) returns an error).");

        assert_eq!(
            transposition(5, 2, 426),
            Err(PermutationError::NotBijectiveError)
        );
    }

    #[test]
    fn test_compose() {
        let sigma = transposition(4, 2, 3).unwrap();
        let tau = transposition(4, 1, 2).unwrap();

        println!("Composing two S_4 permutations.");

        assert_eq!(
            compose(&sigma, &tau),
            Permutation::new(
                vec![3, 1, 2, 4]
            )
        );

        println!("Composing permutation with identity.");

        assert_eq!(
            compose(&sigma, &identity(4).unwrap()),
            Ok(sigma)
        );

        println!("Asserting that attempting to compose two permutations from different symmetric groups results in an error.");

        assert_eq!(
            compose(&tau, &identity(5).unwrap()),
            Err(PermutationError::DomainRangeSizeMismatchError)
        );
    }

    #[test]
    fn conjugate_test() {
        println!("Conjugating an S_4 transposition with another S_4 permutation.");

        let tau = Permutation::new(
            vec![1, 3, 4, 2]
        ).unwrap();
        let sigma = transposition(4, 2, 3).unwrap();

        assert_eq!(
            conjugate(&sigma, &tau),
            transposition(4, 3, 4)
        );
    }

    #[test]
    fn sign_test() {
        println!("Computing the sign of the identity function.");

        assert_eq!(identity(17).unwrap().sign(), 1);

        println!("Computing the sign of a transposition.");

        assert_eq!(transposition(19, 13, 7).unwrap().sign(), -1);

        println!("Computing the sign of an odd permutation (composition of three transpositions).");

        assert_eq!(
            compose(
                &(
                    compose(
                        &transposition(426, 12, 14).unwrap(),
                        &transposition(426, 12, 67).unwrap()
                    ).unwrap()
                ),
                &transposition(426, 234, 348).unwrap()
            ).unwrap().sign(),

            -1
        )
    }

    #[test]
    fn test_cycle_form() {
        println!("Compute cycle form of a transposition from S_4.");

        let tau = transposition(4, 2, 3).unwrap();

        assert_eq!(
            tau.to_cycle_form(),
            vec![
                Cycle::new(vec![1], 4).unwrap(),
                Cycle::new(vec![2, 3], 4).unwrap(),
                Cycle::new(vec![4], 4).unwrap()
            ]
        );

        println!("Compute cycle form of a cycle given in permutation form.");

        let sigma1 = Permutation::new(
            vec![2, 3, 4, 5, 6, 7, 1]
        ).unwrap();

        assert_eq!(
            sigma1.to_cycle_form(),
            vec![
                Cycle::new(vec![1, 2, 3, 4, 5, 6, 7], 7).unwrap()
            ]
        );

        let sigma2 = Permutation::new(
            vec![5, 3, 4, 1, 6, 2]
        ).unwrap();

        assert_eq!(
            sigma2.to_cycle_form(),
            vec![
                Cycle::new(vec![1, 5, 6, 2, 3, 4], 6).unwrap()
            ]
        );

        println!("Testing for permutation with more than one cycle.");

        let rho = Permutation::new(
            vec![5, 6, 3, 1, 4, 2]
        ).unwrap();

        assert_eq!(
            rho.to_cycle_form(),
            vec![
                Cycle::new(vec![1, 5, 4], 6).unwrap(),
                Cycle::new(vec![2, 6], 6).unwrap(),
                Cycle::new(vec![3], 6).unwrap()
            ]
        );
    }

    #[test]
    fn test_cycle_string_representation() {
        println!("Test for cycle of length 4 in S_7.");

        let test_cycle = Cycle::new(
            vec![1, 4, 6, 5], 7
        );

        assert_eq!(test_cycle.unwrap().to_string(), String::from("(1 4 6 5)"));

        println!("Test for empty cycle in S_5.");

        let empty_test_cycle = Cycle::new(
            vec![], 5
        );

        assert_eq!(empty_test_cycle.unwrap().to_string(), String::from("()"));
    }

    #[test]
    fn test_permutation_string_representation() {
        println!("Test for S_6 permutation.");

        let sigma = Permutation::new(
            vec![5, 6, 3, 1, 4, 2]
        ).unwrap();

        assert_eq!(sigma.to_string(), "(1 5 4)(2 6)");
    }
}