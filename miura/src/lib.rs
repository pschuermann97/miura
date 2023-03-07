// module declarations
mod poly;
mod vec_helper;
mod permutation;

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
* Module for unit-tests of this project.
*/
#[cfg(test)]
mod tests {
    use super::*;

    use crate::poly::*;
    use crate::vec_helper::*;

    use crate::permutation::*;

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



    // -------------------- tests for polynomial module --------------------




    #[test]
    fn get_coefficient_test() {
        println!("Checking coefficients of integer polynomial.");

        let integer_poly = IntPoly::new(
            &mut vec![2, 3, 2, 1],
            Modulus::None
        );
        assert_eq!(integer_poly.coefficient(0), 2);
        assert_eq!(integer_poly.coefficient(1), 3);
        assert_eq!(integer_poly.coefficient(2), 2);
        assert_eq!(integer_poly.coefficient(3), 1);
        assert_eq!(integer_poly.coefficient(4), 0);
        assert_eq!(integer_poly.coefficient(426), 0);

        println!("Checking coefficients of remainder class ring polynomial.");

        let rem_class_ring_poly = IntPoly::new(
            &mut vec![1, 13, 5, 10],
            Modulus::Some(5)
        );
        assert_eq!(rem_class_ring_poly.coefficient(0), 1);
        assert_eq!(rem_class_ring_poly.coefficient(1), 3);
        assert_eq!(rem_class_ring_poly.coefficient(2), 0);
        assert_eq!(rem_class_ring_poly.coefficient(3), 0);
        assert_eq!(rem_class_ring_poly.coefficient(4), 0);
        assert_eq!(rem_class_ring_poly.coefficient(5184), 0);
    }

    /*
    * Computes the degree of some polynomials,
    * with/without trailing zeros,
    * over integer and remainder class rings.
    */
    #[test]
    fn degree_test() {
        println!("Testing integer polynomial with no trailing zero coefficients.");

        let poly1 = IntPoly::new(
            &mut vec![1, 1, 1, 1],
            Modulus::None
        );
        assert_eq!(poly1.deg(), 3);

        println!("Testing integer polynomial with trailing zero coefficients.");

        let poly2 = IntPoly::new(
            &mut vec![426, 1, 0],
            Modulus::None
        );
        assert_eq!(poly2.deg(), 1);

        println!("Testing remainder class ring polynomial with no trailing zero coefficients.");

        let poly3 = IntPoly::new(
            &mut vec![1, 1, 1, 2],
            Modulus::Some(5)
        );
        assert_eq!(poly3.deg(), 3);

        println!("Testing remainder class ring polynomial with trailing zero coefficients.");

        let poly4 = IntPoly::new(
            &mut vec![1, 4, 5, 10],
            Modulus::Some(5)
        );
        assert_eq!(poly4.deg(), 1);

        println!("Testing integer zero polynomial.");

        let zero_poly = zero_polynomial(Modulus::None);
        assert_eq!(zero_poly.deg(), -1);
    }

    #[test]
    fn add_poly_test() {
        println!("Adding two integer polynomials with no trailing zeros in the sum.");

        let poly1 = IntPoly::new(
            &mut vec![1, 1, 1, 1],
            Modulus::None
        );
        let poly2 = IntPoly::new(
            &mut vec![425, 425, 425, 425],
            Modulus::None
        );

        let result_12_poly = add_poly(&poly1, &poly2).unwrap();

        assert_eq!(
            result_12_poly,
            IntPoly::new(
                &mut vec![426, 426, 426, 426],
                Modulus::None
            )
        );

        println!("Adding two integer polynomials with trailing zeros in the sum.");

        let poly3 = IntPoly::new(
            &mut vec![1, 1, 1, 426],
            Modulus::None
        );
        let poly4 = IntPoly::new(
            &mut vec![1, 1, 1, -426],
            Modulus::None
        );

        let result_34_poly = add_poly(&poly3, &poly4).unwrap();

        assert_eq!(
            result_34_poly,
            IntPoly::new(
                &mut vec![2, 2, 2],
                Modulus::None
            )
        );

        println!("Adding two remainder class ring polynomials with no trailing zeros in the sum.");

        let poly5 = IntPoly::new(
            &mut vec![1, 1, 1, 1],
            Modulus::Some(5)
        );
        let poly6 = IntPoly::new(
            &mut vec![2, 2, 2, 2],
            Modulus::Some(5)
        );

        let result_56_poly = add_poly(&poly5, &poly6).unwrap();

        assert_eq!(
            result_56_poly,
            IntPoly::new(
                &mut vec![3, 3, 3, 3],
                Modulus::Some(5)
            )
        );

        println!("Adding two remainder class ring polynomials with trailing zeros in the sum.");

        let poly7 = IntPoly::new(
            &mut vec![2, 1, 1, 1],
            Modulus::Some(426)
        );
        let poly8 = IntPoly::new(
            &mut vec![425, 425, 425, 425],
            Modulus::Some(426)
        );

        let result_78_poly = add_poly(&poly7, &poly8).unwrap();

        assert_eq!(
            result_78_poly,
            IntPoly::new(
                &mut vec![427],
                Modulus::Some(426)
            )
        );
    }

    #[test]
    fn add_poly_mismatching_moduli_test() {
        println!("Adding two integer polynomials with mismatching moduli.");

        let poly1 = IntPoly::new(
            &mut vec![1, 1, 1, 1],
            Modulus::None
        );
        let poly2 = IntPoly::new(
            &mut vec![425, 425, 425, 425],
            Modulus::Some(426)
        );

        let result_12_poly = add_poly(&poly1, &poly2);

        assert_eq!(
            result_12_poly, Err(PolynomialError::ModulusMismatchError(
                Modulus::None,
                Modulus::Some(426)
            ))
        )
    }

    #[test]
    fn subtract_poly_test() {
        println!("Subtracting a polynomial from itself.");
        
        let poly1 = IntPoly::new(
            &mut vec![1, 1, 1, 426],
            Modulus::None
        );
        assert_eq!(
            subtract_poly(&poly1, &poly1),
            Ok(
                IntPoly::new(
                    &mut vec![],
                    Modulus::None
                )
            )
        );

        println!("Subtracting two different polynomials.");

        let poly2 = IntPoly::new(
            &mut vec![0, 2, 427, 424],
            Modulus::None
        );
        assert_eq!(
            subtract_poly(&poly1, &poly2),
            Ok(
                IntPoly::new(
                    &mut vec![1, -1, -426, 2],
                    Modulus::None
                )
            )
        );
    }

    #[test]
    fn scale_poly_test() {
        println!("Scale a polynomial with a positive number.");
        let poly1 = IntPoly::new(
            &mut vec![1, 1, 1],
            Modulus::None
        );
        assert_eq!(
            poly1.scale(426),
            IntPoly::new(
                &mut vec![426, 426, 426],
                Modulus::None
            )
        );

        println!("Scale a polynomial with a negative number.");
        assert_eq!(
            poly1.scale(-426),
            IntPoly::new(
                &mut vec![-426, -426, -426],
                Modulus::None
            )
        );

        println!("Scale a polynomial with 0.");
        assert_eq!(
            poly1.scale(0),
            IntPoly::new(
                &mut vec![],
                Modulus::None
            )
        )
    }

    #[test]
    fn test_sum_of_polys() {
        println!("Testing with four integer polynomials.");
        
        let poly1 = IntPoly::new(
            &mut vec![1, 1, 426],
            Modulus::None
        );
        let poly2 = IntPoly::new(
            &mut vec![2, 2, 2],
            Modulus::None
        );
        let poly3 = IntPoly::new(
            &mut vec![1, 1, 0],
            Modulus::None
        );
        let poly4 = IntPoly::new(
            &mut vec![0, 0, 0],
            Modulus::None
        );

        let poly_vec = vec![poly1, poly2, poly3, poly4];

        let result_poly = sum_of_polys(&poly_vec);

        assert_eq!(
            result_poly,
            Ok(
                IntPoly::new(
                    &mut vec![4, 4, 428],
                    Modulus::None
                )
            )
        );

        println!("Testing with empty polynomial vector, expecting zero polynomial.");

        assert_eq!(
            sum_of_polys(&vec![]),
            Ok(zero_polynomial(Modulus::None))
        )
    }

    #[test]
    fn test_multiply_poly() {
        println!("Testing with two integer polynomials.");

        let poly1 = IntPoly::new(
            &mut vec![1, 2, 1],
            Modulus::None
        );

        let poly2 = IntPoly::new(
            &mut vec![0, 4, 0, 1],
            Modulus::None
        );

        assert_eq!(
            multiply_poly(&poly1, &poly2),
            Ok(
                IntPoly::new(
                    &mut vec![0, 4, 8, 5, 2, 1],
                    Modulus::None
                )
            )
        );

        // polynomial multiplication should be commutative
        assert_eq!(
            multiply_poly(&poly2, &poly1),
            Ok(
                IntPoly::new(
                    &mut vec![0, 4, 8, 5, 2, 1],
                    Modulus::None
                )
            )
        );
    }

    #[test]
    fn test_product_of_polys() {
        println!("Testing with three integer polynomials.");

        let poly1 = IntPoly::new(
            &mut vec![0, 1, 1],
            Modulus::None
        );
        let poly2 = IntPoly::new(
            &mut vec![1, 1, 1],
            Modulus::None
        );
        let poly3 = IntPoly::new(
            &mut vec![1, 1, 2],
            Modulus::None
        );

        assert_eq!(
            product_of_polys(&vec![poly1, poly2, poly3]),
            Ok(
                IntPoly::new(
                    &mut vec![0, 1, 3, 6, 7, 5, 2],
                    Modulus::None
                )
            )
        );
    }

    #[test]
    fn test_poly_power() {
        println!("Testing with integer polynomial and exponent 2.");

        let poly = IntPoly::new(
            &mut vec![1, 1],
            Modulus::None
        );
        assert_eq!(
            poly_power(&poly, 2),
            Ok(
                IntPoly::new(
                    &mut vec![1, 2, 1],
                    Modulus::None
                )
            )
        );

        println!("Testing with integer polynomial and exponent 3.");

        assert_eq!(
            poly_power(&poly, 3),
            Ok(
                IntPoly::new(
                    &mut vec![1, 3, 3, 1],
                    Modulus::None
                )
            )
        );

        println!("Asserting that remainder class ring polynomial to the power of 0 is over correct ring.");

        let rem_class_poly = IntPoly::new(
            &mut vec![1, 1],
            Modulus::Some(5)
        );

        assert_eq!(
            poly_power(&rem_class_poly, 0),
            Ok(one_polynomial(Modulus::Some(5)))
        );
    }

    #[test]
    fn test_string_representation() {
        let poly1 = IntPoly::new(
            &mut vec![1, 2, 1, 0],
            Modulus::None
        );

        println!("{}", poly1.to_string());
    }



    // -------------------- end of tests for polynomial module --------------------



    // -------------------- tests for permutations module --------------------------

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
            Err(PermutationError::ImageOutOfRangeError)
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
            Err(PermutationError::ImageOutOfRangeError)
        );
    }


    // -------------------- end of tests for permutations module -------------------



    // -------------------- tests for vector helper functions ---------------------



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
}
