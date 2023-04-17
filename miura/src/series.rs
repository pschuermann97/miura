/// How many summands of a power series are computed.
/// Small number means higher efficiency 
/// while many iterations yield higher precision.
pub const NUM_ITERATIONS: usize = 100;

/// Computes the exponential function via power series expansion.
/// WIP function, currently lacking suitable floating point precision.
pub fn exp(x: f64) -> f64 {
    /*
    * Nominator and denominator of the fraction are computed iteratively,
    * to avoid redundant computations and thus improve efficiency.
    * The respective variables are initialized as follows.
    * With these initialization values, we have already computed the first iteration.
    */
    let mut x_pow_k = 1.0;
    let mut k_factorial = 1.0;
    let mut result = 1.0;

    for k in 1..NUM_ITERATIONS {
        x_pow_k *= x;
        k_factorial = k_factorial * (k as f64);

        result += x_pow_k / k_factorial;
    }

    result
}