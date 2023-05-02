//! An API to perform arithmetic operations on integer polynomials.



use std::cmp::max; // maximum function
use crate::vec_helper::*; // helper functions for operating on coefficient vectors

/// Models a polynomial a_0 + a_1 * X + ... + a_n * X^n with either integer coefficients 
/// or coefficients from a remainder class ring. 
/// 
/// Polynomials are stored as coefficient vectors
/// and the coefficient for the highest-degree monomial is guaranteed to be != 0.
#[derive(PartialEq, Debug, Clone)]
pub struct IntPoly {
    /// Vector of coefficients of the polynomial,
    /// coeff[i] is the coefficient for X^i.
    coefficients: Vec<i32>,
    /// For polynomials over a remainder class ring Z_q, this is Some(q).
    /// For polynomials over integers, this is None.
    modulus: Modulus
}

impl IntPoly {
    /// Constructor, creates a struct instance modeling a new polynomial.
    ///
    /// Note that trailing zeros are cut,
    /// i.e. 1 + X + 0X^2 + 4X^3 + 0X^4 would become 1 + X + 0X^2 + 4X^3.
    ///
    /// For a polynomial over a remainder class ring Z/qZ,
    /// this means that all trailing multiples of q are cut,
    /// i.e. 1 + qX + 3qX^2 would become 1.
    ///
    /// Any coefficients for monomials with higher degrees than the explicitly listed ones are 0. 
    pub fn new(coeff:&mut Vec<i32>, md: Modulus) -> IntPoly {
        // trim trailing zeros/ multiples of modulus
        remove_trailing_zeros(coeff, md);
        
        IntPoly {
            coefficients: coeff.to_vec(),
            modulus: md
        }
    }

    /// Returns the coefficient for the monomial with the passed exponent.
    ///
    /// For polynomials over remainder class ring Z/qZ, 
    /// we reduce the coefficient to the standard representative system {0, ..., q-1}.
    pub fn coefficient(self: &Self, exponent: usize) -> i32 {
        /*
        * Coefficients that are not explicitly listed in the self.coefficients vector are 0.
        * We capture the edge case of the zero polynomial 
        * (coefficients vector is empty, thus all coefficients are 0)
        * here to prevent a runtime error when comparing the length of the coefficients vector - 1
        * to the passed exponent (which is an usize integer, thus non-negative) down below.
        */
        if self.coefficients.len() == 0 {
            return 0;
        }
        
        // coefficients that are not explicitly listed in the vector are 0
        if exponent > self.coefficients.len() - 1 {
            return 0;
        }
        
        match self.modulus {
            Modulus::Some(q) => self.coefficients[exponent] % q,
            Modulus::None => self.coefficients[exponent]
        }
    }
    
    /// Computes the degree of the passed polynomial.
    /// Exploits the fact that trailing zeros are cut from the polynomial upon instantiation,
    /// i.e. 1 + X + 0X^2 + 4X^3 + 0X^4 becomes 1 + X + 0X^2 + 4X^3.
    ///
    /// Note that in this library, the degree of the zero polynomial is -1,
    /// some literature has it as negative infinity.
    pub fn deg(self: &Self) -> i32 {
        if self.coefficients.len() == 0 { -1 } 
        /*
        * This value is always greater then -1 <=> self.coefficients.len() is greater 0.
        * The latter is guaranteed in this else block so unwrap will never panic.
        */
        else { (self.coefficients.len() - 1).try_into().unwrap() }
    }

    /// Scales the polynomial with the passed scale factor,
    /// i.e. multiplies all the coefficients with it.
    /// The result is returned as a new IntPoly instance,
    /// the original polynomial is not changed.
    pub fn scale(self: &Self, scale_factor: i32) -> IntPoly {
        IntPoly::new( // this call removes trailing zeros from the passed vector automatically
            &mut scale_vector(&self.coefficients, scale_factor),
            self.modulus
        )
    }

    /// Returns the additive inverse of the passed polynomial.
    pub fn additive_inverse(self: &Self) -> IntPoly {
        self.scale(-1)
    }

    /// Computes a string representation of this polynomial,
    /// looking like "1X^0 + 2X^1 + 1X^2"
    pub fn to_string(self: &Self) -> String {
        /*
        * Special case: representation of polynomial with empty coefficients vector 
        * is not the empty string "" but "0".
        */
        if self.coefficients.len() == 0{
            return String::from("0");
        }
        
        let mut string_representation = String::new();
        
        for (i, &a_i) in self.coefficients.iter().enumerate() {
            // add a proper description of each monomial
            if a_i != 0 {
                string_representation.push_str(
                    &(a_i.to_string() + "X^" + &i.to_string())
                );
            }

            // add a "+" between the monomials
            string_representation.push_str(" + ");
        }

        // remove final "+"
        string_representation = string_representation[
            0..string_representation.len()-3
        ].to_string();

        return string_representation;
    }
}

/// Returns the sum of the two passed polynomials.
/// Trailing zeros of the sum are cut in the process.
///
/// If the moduli of the polynomials do not match, the function returns an error. 
pub fn add_poly(poly1: &IntPoly, poly2: &IntPoly) -> Result<IntPoly, PolynomialError> {
    // two polynomials with non-matching moduli cannot be added meaningfully
    if poly1.modulus != poly2.modulus {
        return Err(
            PolynomialError::ModulusMismatchError(poly1.modulus, poly2.modulus)
        );
    }
    
    // compute the (maximum possible) degree = number of coefficients of the resulting polynomial
    let result_len = max(poly1.coefficients.len(), poly2.coefficients.len());

    let mut result_coeffs = vec![];

    // computing the coefficients
    for i in 0..result_len {
        result_coeffs.push(poly1.coefficient(i) + poly2.coefficient(i));
    }

    /*
    * Remove trailing zeros from the result polynomial.
    * 
    * When adding e.g. X^2 + 1 and -X^2 + 1 over Z, the sum is 2 
    * and has a lower degree than the summands.
    *
    * When adding e.g. 3X^2 + X and 2X^2 + 2X over Z/5Z, 
    * the sum is 5X^2 + 3X = 3X and has a lower degree than the summands.
    */
    remove_trailing_zeros(&mut result_coeffs, poly1.modulus); // at this point, the polynomials can be assumed to have the same modulus

    // creating result polynomial 
    let result_poly = IntPoly::new(
        &mut result_coeffs,
        poly1.modulus 
    );

    Ok(result_poly)
}

/// Computes the sum of n polynomials which are passed as a vector of length n.
/// Trailing zeros of the sum are cut in the process.
///
/// If the moduli of the polynomials do not match, the function returns an error. 
///
/// If the passed vector is empty, the zero polynomial is returned.
pub fn sum_of_polys(poly_vec: &Vec<IntPoly>) -> Result<IntPoly, PolynomialError> {
    // empty sum of polynomials is integer zero polynomial
    if poly_vec.len() == 0 {
        return Ok(zero_polynomial(Modulus::None));
    }
    
    // from here, we can assume that poly_vec contains at least one polynomial

    let mut result = zero_polynomial(poly_vec[0].modulus);

    for poly in poly_vec.iter() { // elements of iterators are references
        // "?"-operator: error is returned to the caller, for situation-dependent error handling
        result = add_poly(&result, poly)?; 
    }

    Ok(result)
}

/// Returns the difference poly1 - poly2 of the two passed polynomials.
/// Trailing zeros of the difference are cut in the process.
///
/// If the moduli of the polynomials do not match, the function returns an error. 
pub fn subtract_poly(poly1: &IntPoly, poly2: &IntPoly) -> Result<IntPoly, PolynomialError> {
    add_poly(poly1, &poly2.additive_inverse())
}

/// Returns the product of the two passed polynomials.
/// Trailing zeros of the product are cut in the process.
///
/// If the moduli of the polynomials do not match, the function returns an error.
pub fn multiply_poly(poly1: &IntPoly, poly2: &IntPoly) -> Result<IntPoly, PolynomialError> {
    // two polynomials with non-matching moduli cannot be multiplied meaningfully
    if poly1.modulus != poly2.modulus {
        return Err(
            PolynomialError::ModulusMismatchError(poly1.modulus, poly2.modulus)
        );
    }

    /*
    * At this point we can assume all the polynomials to have the same modulus.
    * We introduce this binding for better readability.
    */
    let the_modulus = poly1.modulus;

    /*
    * The implementation of this function exploits the distributive law for polynomials.
    * So a product of two polynomials f (with m monomials) and g (with degree n)
    * is computed as a sum of m degree-n polynomials.
    *
    * To illustrate this, consider f(X) = X^2 + 2X + 1 with m=3 monomials
    * and g(X) = X^3 + 4X with degree n=3.
    * We have (X^2 + 2X + 1) * (X^3 + 4X) = X^2 * (X^3 + 4X) + 2X * (X^3 + 4X) + 1 * (X^3 + 4X).
    *
    * A product of a monomial aX^i with a polynomial p can be computed
    * by inserting i zeros at the beginning of the coefficient vector of p
    * and then scaling the resulting vector by a (i.e. multiplying every entry by a).
    */

    // create vector to store the m polynomials to add
    let mut poly_vec: Vec<IntPoly> = vec![];

    // compute intermediate degree-n polynomials aX^i * p as described above
    for i in 0..poly1.deg()+1 {
        poly_vec.push(
            IntPoly::new(
                // compute aX^i * p by scaling the coefficients of X^i * p with a
                &mut scale_vector(
                    /*
                    * Compute X^i * p by shifting the coefficients of p
                    * by the degree i of the monomial X^i.
                    */
                    &shift_vector(
                        &poly2.coefficients, 
                        /*
                        * Conversion of i32 to usize will never return Error variant
                        * since i can be guaranteed to be non-negative.
                        */
                        i.try_into().unwrap()
                    ), 
                    // unwrap should not panic, see above
                    poly1.coefficient(i.try_into().unwrap())
                ),
                the_modulus
            )
        );
    }

    sum_of_polys(&poly_vec)
}

/// Computes the product of n polynomials which are passed as a vector of length n.
/// Trailing zeros of the product are cut in the process.
///
/// If the moduli of the polynomials do not match, the function returns an error. 
///
/// If the passed vector is empty, the one polynomial is returned.
pub fn product_of_polys(poly_vec: &Vec<IntPoly>) -> Result<IntPoly, PolynomialError> {
    // empty product of polynomials is the (integer) one polynomial
    if poly_vec.len() == 0 {
        return Ok(one_polynomial(Modulus::None));
    }

    // from here, we can assume that poly_vec contains at least one polynomial

    let mut result = one_polynomial(poly_vec[0].modulus);

    for poly in poly_vec.iter() { // elements of iterators are references
        // "?"-operator: error is returned to the caller, for situation-dependent error handling
        result = multiply_poly(&result, poly)?; 
    }

    Ok(result)
}

/// Takes the passed polynomial to the power 
/// determined by the passed positive exponent.
pub fn poly_power(poly: &IntPoly, exponent: usize) -> Result<IntPoly, PolynomialError> {
    /*
    * Need to catch this special case here since
    * product_of_polys always returns the integer zero polynomial
    * when called with an empty polynomial vector.
    * So p^0 for a remainder class ring polynomial would yield a wrong result 
    * without the following if-clause. 
    */
    if exponent == 0 {
        return Ok(one_polynomial(poly.modulus));
    }
    
    // create vector of exponent copies of the passed polynomial
    let mut poly_vec: Vec<IntPoly> = vec![];
    for _ in 0..exponent {
        poly_vec.push(poly.clone());
    }

    product_of_polys(&poly_vec) // product_of_polys might throw a PolynomialError
}  

/// Returns the zero polynomial with the passed Modulus.
pub fn zero_polynomial(md: Modulus) -> IntPoly {
    IntPoly::new(
        &mut vec![],
        md
    )
}

/// Returns the one polynomial with the passed Modulus.
pub fn one_polynomial(md: Modulus) -> IntPoly {
    IntPoly::new(
        &mut vec![1],
        md
    )
}



/// A modulus for a remainder class ring.
/// Implementation for the binary equals-operator is generated automatically using derived traits.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Modulus {
    Some(i32),
    None
}

/// Models the different error types that can occur when working with polynomials.
#[derive(Debug, PartialEq)]
pub enum PolynomialError {
    /*
    * Returned when trying to do some binary operation for polynomials with different moduli.
    */
    ModulusMismatchError(Modulus, Modulus)
}






/*
* Tests for the polynomial math module.
*/
#[cfg(test)]
mod tests {
    use crate::poly::*;

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

        assert_eq!(poly1.to_string(), "1X^0 + 2X^1 + 1X^2");
    }
}