use crate::vec_helper::scale_vector;
use crate::vec_helper::is_zero_vector;


/// A struct describing a matrix of real numbers
/// with double floating point precision.
///
/// The matrix is stored as a vector of row vectors.
#[derive(PartialEq, Debug, Clone)]
pub struct Matrix {
    rows: Vec<Vec<f32>>
}

impl Matrix {
    /// Constructs a matrix from the passed vector of row vectors.
    /// 
    /// If the passed row vectors do not have the same length,
    /// an error variant is returned.
    pub fn new(rows: Vec<Vec<f32>>) -> Result<Matrix, MatrixError> {
        let expected_row_len = rows[0].len();
        for row in &rows {
            if row.len() != expected_row_len {
                return Err(MatrixError::NonUniformRowLengthError);
            }
        }

        Ok(Matrix {
            rows
        })
    }

    /// Returns the entry in row i and column j of the matrix.
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
    * Switches the rows i and j of the matrix.
    */
    pub fn switch_rows(self: &mut Self, i: usize, j: usize) {
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
    * 
    * This is done by transforming each row into a pivot row.
    * A pivot row is a row that starts with any number of zeros, followed by a non-zero pivot element and then arbitrary elements.
    * The pivot position of its pivot element.
    * A pivot row is called normalized if its pivot element is a one.
    *
    * A matrix is an upper triangular matrix if its rows are ordered by their pivot positions, ascendingly.
    */
    pub fn to_upper_triangular(self: &mut Self) {
        let mut current_pivot_position = 0;

        'pivot_row_creation: for current_row in 0..(self.num_rows()) {
            /*
            * Search for pivot position in current row.
            * If reached end of row : 
            * no pivot position exists in current row, continue with next row
            * (start looking for pivot column from the main diagonal).
            */
            while self.next_row_without_zero_at_beginning_from(current_pivot_position, current_row) == None { 
                current_pivot_position += 1;
                // if reached end of row: no pivot position exists in this row.
                if current_pivot_position==self.num_columns() {
                    /*
                    * Search for pivot position in next row 
                    * should continue from the left end of the row.
                    */
                    current_pivot_position = 0;
                    continue 'pivot_row_creation;
                }
            }
                
            // normalize the row
            self.scale_row(
                current_row, // row index (= column index)
                1.0 / self.entry(current_row, current_pivot_position)
            );

            // eliminate all elements above and below the pivot position
            for r in 0..(self.rows.len()) {
                if r != current_row {
                    self.add_scalar_multiple(
                        r, // row where entry in column current_column should be eliminated
                        - self.entry(r, current_pivot_position), // scale factor for current_column in order to do the elimination via rank-preserving addition operation
                        current_row
                    );
                }
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


/// Type modelling all different kinds of errors
/// that can occur when working with real matrices. 
#[derive(PartialEq, Debug, Clone)]
pub enum MatrixError{
    NonUniformRowLengthError
}


