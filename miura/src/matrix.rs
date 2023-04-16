use crate::vec_helper::scale_vector;
use crate::vec_helper::is_zero_vector;

/*
* A struct describing a matrix of real numbers
* with double floating point precision.
*
* The matrix is stored as a vector of row vectors.
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

    pub fn entry(self: &Self, i: usize, j: usize) -> f32 {
        self.rows[i][j]
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

    /*
    * Returns the number of rows of this matrix.
    */
    pub fn num_rows(self: &Self) -> usize {
        self.rows.len()
    }

    /*
    * Returns the number of columns of this matrix.
    */
    pub fn num_columns(self: &Self) -> usize {
        /*
        * Due to constraint checks in the constructor,
        * we can assume here that all rows of the matrix have equal size.
        */
        self.rows[0].len()
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
    * I.e. row i is replaced by row i + a * row j
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



    // -------------------- end of row operations -----------------------



    /*
    * Transforms the matrix to its upper triangular form.
    */
    pub fn to_upper_triangular(self: &mut Self) {
        'column_elimination: for current_column in 0..(self.num_rows()) {
            /*
            * Check whether there is a row that has a non-zero number in the current column
            * on/below the diagonal.
            */
            match self.next_row_without_zero_at_beginning_from(current_column, current_column) {
                Some(k) => {
                    // swap row with index current_column and the found row
                    self.swap_rows(current_column, k);
                    
                    // create a 1 on the diagonal
                    self.scale_row(
                        current_column, // row index (= column index)
                        1.0 / self.rows[current_column][current_column]
                    );

                    // eliminate all elements below the diagonal
                    for r in (current_column+1)..(self.rows.len()) {
                        self.add_scalar_multiple(
                            r, // row where entry in column current_column should be eliminated
                            - self.rows[r][current_column], // scale factor for current_column in order to do the elimination via rank-preserving addition operation
                            current_column
                        );

                        Self::display_matrix(self);
                    }
                }
                // if not, continue with next column
                None => continue 'column_elimination
            }
        }
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

    /*
    * Beginning search from row i, 
    * this method returns the index of the first row of the matrix having no 0 at position (column) j.
    * If no such row exists, the None variant is returned.
    */
    pub fn next_row_without_zero_at_beginning_from(self: &Self, j: usize, i: usize) -> Option<usize> {
        for k in i..self.num_rows() {
            if self.rows[k][j] != 0.0 {
                return Some(k);
            }
        }
        None
    }

    /*
    * Beginning search from row i,
    * this method returns the index of the first row of the matrix that is not a zero row.
    * If no such row exists, the None variant is returned.
    */
    pub fn next_non_zero_row_beginning_from(self: &Self, i: usize) -> Option<usize> {
        for k in i..self.num_rows() {
            if !self.is_zero_row(k) {
                return Some(k);
            }
        }
        None
    }



    // -------------- end of helper functions for computing upper triangular matrix ------------



    // -------------- debug only functions ---------------------------------------



    pub fn display_matrix(a: &Matrix) {
        for i in 0..a.num_rows() {
            for j in 0..a.num_columns() {
                print!("{} ", a.entry(i, j));
            }
            println!("\n");
        }
        println!("------------------------");
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