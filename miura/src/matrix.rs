//! A work-in-progress module 
//! that allows to perform operations on real matrices.



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
    
    /// Returns the i-th row of the matrix as a vector.
    pub fn row(self: &Self, i: usize) -> Vec<f32> {
        self.rows[i].to_vec()
    }

    /// Returns the j-th column of the matrix as a vector.
    pub fn column(self: &Self, j: usize) -> Vec<f32> {
        let mut result = Vec::<f32>::new();

        // append i-th element of every row
        for i in 0..self.rows.len() {
            result.push(self.rows[i][j]);
        }

        result
    }

    /// Returns the number of rows of this matrix.
    pub fn num_rows(self: &Self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns of this matrix.
    pub fn num_columns(self: &Self) -> usize {
        /*
        * Due to constraint checks in the constructor,
        * we can assume here that all rows of the matrix have equal size.
        */
        self.rows[0].len()
    }

    

    // ---------------- row operations -------------------



    /// Scales row i of the matrix with the scale factor c.
    /// Note that this operation is rank-preserving if and only if c != 0.
    pub fn scale_row(self: &mut Self, i: usize, c: f32) {
        self.rows[i] = scale_vector(&(self.rows[i]), c);
    }

    /// Switches the rows i and j of the matrix.
    pub fn switch_rows(self: &mut Self, i: usize, j: usize) {
        let tmp = self.row(i).clone();
        self.rows[i] = self.rows[j].clone();
        self.rows[j] = tmp;
    }

    /// Adds a times row j to row i of the matrix.
    /// I.e. row i is replaced by row i + a * row j
    /// Row j remains unchanged.
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



    /// Transforms the matrix to its upper triangular form.
    ///
    /// This is done by transforming each row into a pivot row.
    /// A pivot row is a row that starts with any number of zeros, followed by a non-zero pivot element and then arbitrary elements.
    /// The pivot position of its pivot element.
    /// A pivot row is called normalized if its pivot element is a 1.
    ///
    /// A matrix is an upper triangular matrix if its rows are ordered by their pivot positions, ascendingly.
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



    /// Determines whether the i-th row of the matrix is a zero row.
    pub fn is_zero_row(self: &Self, i: usize) -> bool {
        is_zero_vector(self.row(i))
    }

    /// Determines whether the j-th column of the matrix is a zero column.
    pub fn is_zero_column(self: &Self, j: usize) -> bool {
        is_zero_vector(self.column(j))
    }

    /// Beginning search from row i, 
    /// this method returns the index of the first row of the matrix having no 0 at position (column) j.
    /// If no such row exists, the None variant is returned.
    pub fn next_row_without_zero_at_beginning_from(self: &Self, j: usize, i: usize) -> Option<usize> {
        for k in i..self.num_rows() {
            if self.rows[k][j] != 0.0 {
                return Some(k);
            }
        }
        None
    }

    /// Beginning search from row i,
    /// this method returns the index of the first row of the matrix that is not a zero row.
    /// If no such row exists, the None variant is returned.
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


    /// Prints the matrix to the console for debug purposes.
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








#[cfg(test)]
mod tests {
    use crate::matrix::*;

    #[test]
    fn matrix_construction_test() {
        println!("Creating test 3x3 matrix.");

        let test_matrix = Matrix::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 426.0]
            ]
        );

        assert!(test_matrix.is_ok());

        println!("Attempting to create a matrix with rows of non-uniform length.");

        let errornous_matrix = Matrix::new(
            vec![
                vec![1.0, 2.0],
                vec![426.0, 426.0, 426.0]
            ]
        );

        assert_eq!(errornous_matrix, Err(MatrixError::NonUniformRowLengthError));
    }



    #[test]
    fn rows_and_columns_test() {
        println!("Creating test 3x3 matrix.");

        let test_matrix = Matrix::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 426.0]
            ]
        ).unwrap();

        println!("Asserting that 2nd row is extracted correctly.");

        assert_eq!(test_matrix.row(1), vec![4.0, 5.0, 6.0]);

        println!("Asserting that 3rd column is extracted correctly.");

        assert_eq!(test_matrix.column(2), vec![3.0, 6.0, 426.0]);
    }

    #[test]
    fn scale_row_test() {
        println!("Creating 3x4 test matrix.");

        let mut test_matrix = Matrix::new(
            vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 426.0],
                vec![9.0, 10.0, 11.0, 12.0]
            ]
        ).unwrap();

        println!("Asserting that second row can be correctly scaled by 1.5.");

        test_matrix.scale_row(1, 1.5);
        assert_eq!(
            test_matrix.row(1), vec![7.5, 9.0, 10.5, 639.0]
        );
    }

    #[test]
    fn switch_rows_test() {
        println!("Creating 4x3 test matrix.");

        let mut test_matrix = Matrix::new(
            vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 426.0, 5.0],
                vec![426.0, 8.0, 426.0],
                vec![10.0, 11.0, 12.0]
            ]
        ).unwrap();

        println!("Asserting that rows 1 and 2 can be correctly swapped.");

        test_matrix.switch_rows(1, 2);
        assert_eq!(
            test_matrix,
            Matrix::new(
                vec![
                    vec![1.0, 2.0, 3.0],
                    vec![426.0, 8.0, 426.0],
                    vec![4.0, 426.0, 5.0],
                    vec![10.0, 11.0, 12.0]
                ]
            ).unwrap()
        )
    }

    #[test]
    fn add_scalar_multiple_test() {
        println!("Creating test 3x2 matrix.");

        let mut test_matrix = Matrix::new(
            vec![
                vec![1.0, 3.0],
                vec![0.0, 2.0],
                vec![4.0, 426.0]
            ]
        ).unwrap();

        println!("Adding 3.2 times row 0 to row 1.");

        test_matrix.add_scalar_multiple(1, 3.2, 0);
        assert_eq!(
            test_matrix,    
            Matrix::new(
                vec![
                    vec![1.0, 3.0],
                    vec![3.2, 11.6],
                    vec![4.0, 426.0]
                ]
            ).unwrap()
        );
    }

    #[test]
    fn is_zero_row_column_test(){
        println!("Creating 3x4 test matrix.");

        let test_matrix = Matrix::new(
            vec![
                vec![1.0, 2.0, 426.0, 0.0],
                vec![0.0, 0.0, 0.0, 0.0],
                vec![9.0, 10.0, 11.0, 0.0]
            ]
        ).unwrap();

        println!("Asserting that zero row is correctly classified.");

        assert_eq!(test_matrix.is_zero_row(1), true);

        println!("Asserting that non-zero row is correctly classified.");

        assert_eq!(test_matrix.is_zero_row(2), false);

        println!("Asserting that zero column is correctly classified.");

        assert_eq!(test_matrix.is_zero_column(3), true);

        println!("Asserting that non-zero column is correctly classified.");

        assert_eq!(test_matrix.is_zero_column(0), false);
    }

    #[test]
    fn test_first_row_without_zero_at_beginning_from() {
        println!("Creating 3x3 test matrix.");

        let test_matrix = Matrix::new(
            vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0]
            ]
        ).unwrap();

        println!("Searching for non zero elements in different columns.");

        assert_eq!(test_matrix.next_row_without_zero_at_beginning_from(0, 0), Some(0));
        assert_eq!(test_matrix.next_row_without_zero_at_beginning_from(0, 1), None);
        assert_eq!(test_matrix.next_row_without_zero_at_beginning_from(1, 0), Some(2));
        assert_eq!(test_matrix.next_row_without_zero_at_beginning_from(2, 0), None);
    }

    #[test]
    fn test_first_non_zero_row_beginning_from() {
        println!("Creating 5x3 test matrix.");

        let test_matrix = Matrix::new(
            vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 426.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
            ]
        ).unwrap();

        println!("Searching for next non-zero row from various different starting rows.");

        assert_eq!(test_matrix.next_non_zero_row_beginning_from(0), Some(0));
        assert_eq!(test_matrix.next_non_zero_row_beginning_from(1), Some(2));
        assert_eq!(test_matrix.next_non_zero_row_beginning_from(2), Some(2));
        assert_eq!(test_matrix.next_non_zero_row_beginning_from(3), None);
        assert_eq!(test_matrix.next_non_zero_row_beginning_from(4), None);
    }

    #[test]
    fn test_to_upper_triangular() {
        println!("Creating 3x4 test matrix.");

        let mut test_matrix1 = Matrix::new(
            vec![
                vec![1.0, 1.0, -1.0, 0.0],
                vec![2.0, 1.0, 0.0, 1.0],
                vec![3.0, 1.0, 2.0, 0.0]
            ]
        ).unwrap();

        println!("Assert that upper triangular form of the first test matrix is correctly computed.");

        test_matrix1.to_upper_triangular();
        assert_eq!(
            test_matrix1,
            Matrix::new(
                vec![
                    vec![1.0, 0.0, 0.0, 3.0],
                    vec![0.0, 1.0, 0.0, -5.0],
                    vec![0.0, 0.0, 1.0, -2.0]
                ]
            ).unwrap()
        );

        println!("Creating 3x5 test matrix.");

        let mut test_matrix2 = Matrix::new(
            vec![
                vec![3.0, -3.0, 3.0, 6.0, 3.0],
                vec![1.0, -1.0, -3.0, 0.0, -8.0],
                vec![2.0, -2.0, -2.0, 2.0, 5.0]
            ]
        ).unwrap();

        println!("Assert that upper triangular form of the second test matrix is correctly computed.");

        test_matrix2.to_upper_triangular();
        assert_eq!(
            test_matrix2,
            Matrix::new(
                vec![
                    vec![1.0, -1.0, 0.0, 1.5, 0.0],
                    vec![0.0, 0.0, 1.0, 0.5, 0.0],
                    vec![0.0, 0.0, 0.0, 0.0, 1.0]
                ]
            ).unwrap()
        );

        println!("Creating 4x5 test matrix.");

        let mut test_matrix3 = Matrix::new(
            vec![
                vec![1.0, 1.0, 1.0, 0.0, 3.0],
                vec![-11.0, 1.0, 9.0, 2.0, -15.0],
                vec![3.0, 0.0, -3.0, 0.0, 3.0],
                vec![8.0, 2.0, -4.0, -1.0, 13.0]
            ]
        ).unwrap();

        println!("Assert that upper triangular form of the third test matrix is correctly computed.");

        test_matrix3.to_upper_triangular();
        assert_eq!(
            test_matrix3,
            Matrix::new(
                vec![
                    vec![1.0, 0.0, 0.0, 0.0, 2.0],
                    vec![0.0, 1.0, 0.0, 0.0, 0.0],
                    vec![0.0, 0.0, 1.0, 0.0, 1.0],
                    vec![0.0, 0.0, 0.0, 1.0, -1.0]
                ]
            ).unwrap()
        );

        println!("Creating 3x4 test matrix.");

        let mut test_matrix4 = Matrix::new(
            vec![
                vec![3.0, 0.0, 3.0, 0.0],
                vec![3.0, 1.0, 1.0, 0.0],
                vec![2.0, 1.0, 0.0, 0.0]
            ]
        ).unwrap();

        println!("Assert that upper triangular form of the fourth test matrix is correctly computed.");

        test_matrix4.to_upper_triangular();
        assert_eq!(
            test_matrix4,
            Matrix::new(
                vec![
                    vec![1.0, 0.0, 1.0, 0.0],
                    vec![0.0, 1.0, -2.0, 0.0],
                    vec![0.0, 0.0, 0.0, 0.0]
                ]
            ).unwrap()
        );

        println!("Assert idempotency of operation.");

        // at this point, the matrix already is in upper triangular form
        test_matrix4.to_upper_triangular();

        test_matrix4.to_upper_triangular();
        assert_eq!(
            test_matrix4,
            Matrix::new(
                vec![
                    vec![1.0, 0.0, 1.0, 0.0],
                    vec![0.0, 1.0, -2.0, 0.0],
                    vec![0.0, 0.0, 0.0, 0.0]
                ]
            ).unwrap()
        );
    }
}