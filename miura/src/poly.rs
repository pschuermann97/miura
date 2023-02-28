use std::cmp::max; // maximum function

/*
* Models a polynomial with either integer coefficients 
* or coefficients from a remainder class ring. 
*
* The highest-degree monomial is guaranteed to have a coefficient != 0.
*/
pub struct IntPoly {
    /*
    * Vector of coefficients of the polynomial,
    * coeff[i] is the coefficient for X^i.
    */
    coefficients: Vec<i32>,
    /*
    * For polynomials over a remainder class ring Z_q, this is Some(q).
    * For polynomials over integers, this is None.
    */
    modulus: Modulus
}

impl IntPoly {
    /*
    * Constructor, creates a struct instance modeling a new polynomial.
    * 
    * Note that trailing zeros are cut,
    * i.e. 1 + X + 0X^2 + 4X^3 + 0X^4 would become 1 + X + 0X^2 + 4X^3.
    *
    * For a polynomial over a remainder class ring Z/qZ,
    * this means that all trailing multiples of q are cut,
    * i.e. 1 + qX + 3qX^2 would become 1.
    *
    * Any coefficients for monomials with higher degrees than the explicitly listed ones are 0. 
    */
    pub fn new(coeff:&mut Vec<i32>, md: Modulus) -> IntPoly {
        // trim trailing zeros/ multiples of modulus
        remove_trailing_zeros(coeff, md);
        
        IntPoly {
            coefficients: coeff.to_vec(),
            modulus: md
        }
    }

    /*
    * Returns the coefficient for the monomial with the passed exponent.
    *
    * For polynomials over remainder class ring Z/qZ, 
    * we reduce the coefficient to the standard representative system {0, ..., q-1}.
    */
    pub fn coefficient(self: &Self, exponent: usize) -> i32 {
        // coefficients that are not explicitly listed in the vector are 0
        if exponent > self.coefficients.len() - 1 {
            return 0;
        }
        
        match self.modulus {
            Modulus::Some(q) => self.coefficients[exponent] % q,
            Modulus::None => self.coefficients[exponent]
        }
    }
    
    /*
    * Computes the degree of the passed polynomial.
    * Exploits the fact that trailing zeros are cut from the polynomial upon instantiation,
    * i.e. 1 + X + 0X^2 + 4X^3 + 0X^4 becomes 1 + X + 0X^2 + 4X^3.
    */
    pub fn deg(self: &Self) -> usize {
        self.coefficients.len() - 1
    }
}

/*
* Returns the sum of the two passed polynomials.
* Trailing zeros of the sum are cut in the process.
*
* If the moduli of the polynomials do not match, the function returns an error. 
*/
pub fn add_poly(poly1: &IntPoly, poly2: &IntPoly) -> Result<IntPoly, PolynomialError> {
    // two polynomials with non-matching moduli cannot be added meaningfully
    if poly1.modulus != poly2.modulus {
        return Err(
            PolynomialError::ModulusMismatchError(poly1.modulus, poly2.modulus)
        );
    }
    
    let result_len = max(poly1.deg(), poly2.deg());

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

/*
* Removes the trailing zeros/ multiples of the passed modulus from the passed vector,
* e.g. vec![2, 3, 0, 0] over modulus None becomes vec![2, 3]
* and vec![2, 4, 5, 5] over modulus Some(5) becomes vec![2, 4].
*
*/
fn remove_trailing_zeros(vec: &mut Vec<i32>, modulus: Modulus) {
    let mut n = vec.len();
    while 
        (modulus == Modulus::None && vec[n-1] == 0 )
        || {
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
* A modulus for a remainder class ring.
* Implementation for the binary equals-operator is generated automatically using derived traits.
*/
#[derive(PartialEq, Copy, Clone)]
pub enum Modulus {
    Some(i32),
    None
}

/*
* Models the different error types that can occur when working with polynomials.
*/
pub enum PolynomialError {
    /*
    * Returned when trying to do some binary operation for polynomials with different moduli.
    */
    ModulusMismatchError(Modulus, Modulus)
}