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

/*
* A modulus for a remainder class ring.
*/
pub enum Modulus {
    Some(i32),
    None
}

impl IntPoly {
    /*
    * Constructor, creates a struct instance modeling a new polynomial.
    * 
    * Note that trailing zeros are cut,
    * i.e. 1 + X + 0X^2 + 4X^3 + 0X^4 would become 1 + X + 0X^2 + 4X^3.
    */
    pub fn new(coeff:&mut Vec<i32>, md: Modulus) -> IntPoly {
        // trim trailing zeros
        let mut n = coeff.len();
        while coeff[n-1] == 0 {
            coeff.pop();
            n = coeff.len(); // coefficients vector was shortened by 1
        }
        
        IntPoly {
            coefficients: coeff.to_vec(),
            modulus: md
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