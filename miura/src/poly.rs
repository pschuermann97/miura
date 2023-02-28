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
        let mut n = coeff.len();
        while 
            (md == Modulus::None && coeff[n-1] == 0 )
            || {
                if let Modulus::Some(x) = md {
                    coeff[n-1] % x == 0
                } else { false }
            }    
        {
            coeff.pop();
            n = coeff.len(); // coefficients vector was shortened by 1
        }
        
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
* A modulus for a remainder class ring.
* Implementation for the binary equals-operator is generated automatically using derived traits.
*/
#[derive(PartialEq)]
pub enum Modulus {
    Some(i32),
    None
}