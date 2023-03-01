use std::cmp::max; // maximum function

/*
* Models a polynomial with either integer coefficients 
* or coefficients from a remainder class ring. 
*
* The highest-degree monomial is guaranteed to have a coefficient != 0.
*/
#[derive(PartialEq, Debug)]
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
    *
    * Note that in this library, the degree of the zero polynomial is -1,
    * some literature has it as negative infinity.
    */
    pub fn deg(self: &Self) -> i32 {
        if self.coefficients.len() == 0 { -1 } 
        /*
        * This value is always greater then -1 <=> self.coefficients.len() is greater 0.
        * The latter is guaranteed in this else block so unwrap will never panic.
        */
        else { (self.coefficients.len() - 1).try_into().unwrap() }
    }

    /*
    * Scales the polynomial with the passed scale factor,
    * i.e. multiplies all the coefficients with it.
    * The result is returned as a new IntPoly instance,
    * the original polynomial is not changed.
    */
    pub fn scale(self: &Self, scale_factor: i32) -> IntPoly {
        IntPoly::new( // this call removes trailing zeros from the passed vector automatically
            &mut self.coefficients.iter().map(|a| scale_factor * a).collect::<Vec<i32>>(),
            self.modulus
        )
    }

    /*
    * Returns the additive inverse of the passed polynomial.
    */
    pub fn additive_inverse(self: &Self) -> IntPoly {
        self.scale(-1)
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

/*
* Returns the difference poly1 - poly2 of the two passed polynomials.
* Trailing zeros of the difference are cut in the process.
*
* If the moduli of the polynomials do not match, the function returns an error. 
*/
pub fn subtract_poly(poly1: &IntPoly, poly2: &IntPoly) -> Result<IntPoly, PolynomialError> {
    add_poly(poly1, &poly2.additive_inverse())
}

pub fn multiply_poly(poly1: &IntPoly, poly2: &IntPoly) -> Result<IntPoly, PolynomialError> {
    // two polynomials with non-matching moduli cannot be multiplied meaningfully
    if poly1.modulus != poly2.modulus {
        return Err(
            PolynomialError::ModulusMismatchError(poly1.modulus, poly2.modulus)
        );
    }

    Ok(zero_polynomial(Modulus::None))
}

/*
* Returns the zero polynomial with the passed Modulus.
*/
pub fn zero_polynomial(md: Modulus) -> IntPoly {
    IntPoly::new(
        &mut vec![],
        md
    )
}

/*
* Returns the one polynomial with the passed Modulus.
*/
pub fn one_polynomial(md: Modulus) -> IntPoly {
    IntPoly::new(
        &mut vec![1],
        md
    )
}



// ---------------- vector helper functions ------------------------



/*
* Removes the trailing zeros/ multiples of the passed modulus from the passed vector,
* e.g. vec![2, 3, 0, 0] over modulus None becomes vec![2, 3]
* and vec![2, 4, 5, 5] over modulus Some(5) becomes vec![2, 4].
*/
fn remove_trailing_zeros(vec: &mut Vec<i32>, modulus: Modulus) {
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



// ---------------- end of vector helper functions ------------------------



/*
* A modulus for a remainder class ring.
* Implementation for the binary equals-operator is generated automatically using derived traits.
*/
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Modulus {
    Some(i32),
    None
}

/*
* Models the different error types that can occur when working with polynomials.
*/
#[derive(Debug, PartialEq)]
pub enum PolynomialError {
    /*
    * Returned when trying to do some binary operation for polynomials with different moduli.
    */
    ModulusMismatchError(Modulus, Modulus)
}