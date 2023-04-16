use crate::vec_helper::scale_vector;

/*
* A struct describing a matrix of real numbers
* with double floating point precision.
*
* The matrix is stored as a vector of row vectors
*/
pub struct Matrix {
    rows: Vec<Vec<f32>>
}

impl Matrix {
    /*
    * Simple constructor to have matrix API that is similar to polynomials and permutations.
    */
    pub fn new(rows: Vec<Vec<f32>>) -> Matrix {
        Matrix {
            rows
        }
    }
    
    /*
    * Returns the i-th row of the matrix as a vector.
    */
    pub fn row(self: &Self, i: usize) -> Vec<f32> {
        self.rows[i].to_vec()
    }

    /*
    * Returns the j-th row of the matrix as a vector.
    */
    pub fn column(self: &Self, j: usize) -> Vec<f32> {
        let mut result = Vec::<f32>::new();

        // append i-th element of every row
        for i in 0..self.rows.len() {
            result.push(self.rows[i][j]);
        }

        result
    }

    

    // ---------------- row operations -------------------



    /*
    * Scales row i of the matrix with the scale factor c.
    * Note that this operation is rank-preserving if and only if c != 0.
    */
    pub fn scale_row(self: &mut Self, i: usize, c: f32) {
        self.rows[i] = scale_vector(&(self.rows[i]), c);
    }
}