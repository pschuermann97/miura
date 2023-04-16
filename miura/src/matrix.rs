use crate::vec_helper::scale_vector;
use crate::vec_helper::is_zero_vector;

/*
* A struct describing a matrix of real numbers
* with double floating point precision.
*
* The matrix is stored as a vector of row vectors
*/
#[derive(PartialEq, Debug, Clone)]
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

    /*
    * Swaps the rows i and j of the matrix.
    */
    pub fn swap_rows(self: &mut Self, i: usize, j: usize) {
        let tmp = self.row(i).clone();
        self.rows[i] = self.rows[j].clone();
        self.rows[j] = tmp;
    }

    /*
    * Adds a times row j to row i of the matrix.
    * Row j remains unchanged.
    */
    pub fn add_scalar_multiple(self: &mut Self, i: usize, a: f32, j: usize) {
        let row_j_scaled = scale_vector(&self.rows[j], a);

        let mut new_row_i = vec![];
        for k in 0..self.rows[i].len() {
            new_row_i.push(
                self.rows[i][k] + row_j_scaled[k]
            );
        }

        self.rows[i] = new_row_i;
    }

    /*
    * Transforms the matrix to its upper triangular form.
    */
    pub fn to_upper_triangular(self: &mut Self) {

    }



    // -------------- helper functions for computing upper triangular matrix ------------



    /*
    * Determines whether the i-th row of the matrix is a zero row.
    */
    pub fn is_zero_row(self: &Self, i: usize) -> bool {
        is_zero_vector(self.row(i))
    }

    /*
    * Determines whether the j-th column of the matrix is a zero column.
    */
    pub fn is_zero_column(self: &Self, j: usize) -> bool {
        is_zero_vector(self.column(j))
    }
}



pub struct LinearEquationSystem{
    coefficient_matrix: Matrix,
    constant_vector: Vec<f32>
}

impl LinearEquationSystem {
    pub fn extended_coefficient_matrix(self: &Self) -> Matrix {
        Matrix::new(
            vec![]
        )
    }
}