use std::error::Error;
use num_traits::Num;

/// A simple *m x n* Matrix implementation, with *m* `rows` and *n* `cols`.
///
/// # Type parameters
/// - `T`: Generic type
///
/// # Examples
///
/// This example uses the `matrix!` macro to add elements to the matrix. Note
/// that the values are added as an array, with sub-arrays corresponding to rows.
///
/// ```
/// # #[macro_use] extern crate algorithms;
/// # use crate::algorithms::data_structures::matrix::*;
/// # fn main() {
/// let mut new_matrix = matrix![[1, 2, 3], [4, 5, 6]];
///
/// // Count rows and columns
/// assert_eq!(new_matrix.rows(), 2);
/// assert_eq!(new_matrix.cols(), 3);
///
/// // Get a specific element
/// assert_eq!(new_matrix.get(1, 2).unwrap(), &6);
///
/// // Modify a specific element
/// new_matrix.set(1, 2, 100);
/// assert_eq!(new_matrix.get(1, 2).unwrap(), &100);
///
/// # }
/// ```
/// 
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

pub trait IsMatrix<T> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get(&self, row: usize, col: usize) -> Option<&T>;
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T>;
    fn set(&mut self, row: usize, col: usize, value: T) -> Option<&T>;
    fn get_row(&self, row: usize) -> Result<RowIterator<T>, Box<dyn Error>>;
    fn get_col(&self, col: usize) -> Result<ColumnIterator<T>, Box<dyn Error>>;
    fn swap_rows(&mut self, row_a: usize, row_b: usize) -> Result<(), Box<dyn Error>>;
    fn swap_cols(&mut self, col_a: usize, col_b: usize) -> Result<(), Box<dyn Error>>;
    fn transpose(&self) -> Matrix<&T>;
    fn apply<F: FnMut(&T)>(&self, func: F);
    fn apply_mut<F: FnMut(&mut T)>(&mut self, func: F);
}

impl<T> Matrix<T> {
    /// Constructs an *m x n* `Matrix`, with *m* `rows` and *n* `cols` 
    /// initialized to zero. 
    ///  
    /// Use `Matrix::from_iter` to build from an iterator.
    /// 
    /// Use `Matrix::from_vec' to build from an existing vector.
    /// 
    /// Use `Matrix::identity` to build an identity matrix.
    /// 
    /// # Returns
    /// Returns a `Matrix<T>` of size `rows` *x* `cols` with all 
    /// values set to zero.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are zero or less.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<i32> = Matrix::new(2, 4);
    /// 
    /// assert_eq!(matrix.get(1, 1).unwrap(), &0);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Default + Num
    {
        Matrix::from_iter(rows, cols, (0..).map(|_| num_traits::zero()))
    }

    /// Constructs an *m x n* `Matrix`, with *m* `rows` and *n* `cols` 
    /// where data is populated from an iterator.  
    /// 
    /// The matrix values are set row by row.  
    /// 
    /// Only `rows * cols` values will be consumed from the iterator.
    /// 
    /// # Returns
    /// Returns a `Matrix<T>` of size `rows` *x* `cols`
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are zero or less.
    /// Panics if the iterator does not have at least `rows * cols` values.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    ///
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    /// assert_eq!(matrix.get(0, 1).unwrap(), &1);
    /// assert_eq!(matrix.get(1, 0).unwrap(), &2);
    /// ```
    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    /// Constructs an *m x n* `Matrix`, with *m* `rows` and *n* `cols` 
    /// where data is populated from an iterator.  
    /// 
    /// The matrix values are set row by row.  
    /// 
    /// Only `rows * cols` values will be consumed from the iterator.
    /// 
    /// # Returns
    /// Returns a `Matrix<T>` of size `rows` *x* `cols`
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are zero or less.
    /// Panics if the iterator does not have at least `rows * cols` values.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let v = vec![1, 2, 3, 4];
    /// let matrix: Matrix<usize> = Matrix::from_vec(2, 2, v);
    ///
    /// assert_eq!(matrix.get(0, 0).unwrap(), &1);
    /// assert_eq!(matrix.get(0, 1).unwrap(), &2);
    /// assert_eq!(matrix.get(1, 0).unwrap(), &3);
    /// ```
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            rows,
            cols,
            data,
        }
    }

    /// Constructs an identity matrix of size `size` *x* `size`.
    /// 
    /// The matrix must be square so only one dimension is needed.
    ///
    /// # Returns
    /// Returns an identiy matrix `Matrix<T>` of size `size` *x* `size`
    /// 
    /// # Panics
    /// Panics if `size` is zero or less.
    /// 
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let identity: Matrix<i32> = Matrix::identity(3);
    /// 
    /// assert_eq!(identity.get(0, 0).unwrap(), &1);
    /// assert_eq!(identity.get(0, 1).unwrap(), &0);
    /// assert_eq!(identity.get(2, 2).unwrap(), &1);
    /// ```
    pub fn identity (size: usize) -> Matrix<T> 
    where
        T: Default + Clone + Num
    {
        let mut data: Vec<T> = vec![num_traits::zero(); size * size];

        for i in 0..size {
            data[i * (size + 1)] = num_traits::one();
        }
        Matrix { rows: size, cols: size, data }
    }
}

impl<T> IsMatrix<T> for Matrix<T> {
    /// Gets the number of rows in a `Matrix`.
    /// 
    /// # Returns
    /// Returns the number of rows in `Matrix`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<usize> = Matrix::new(3, 6);
    ///
    /// assert_eq!(matrix.rows(), 3);
    /// ```
    fn rows(&self) -> usize {
        self.rows
    }

    /// Gets the number of columns in a `Matrix`.
    /// 
    /// # Returns
    /// Returns the number of columns in `Matrix`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<usize> = Matrix::new(3, 6);
    ///
    /// assert_eq!(matrix.cols(), 6);
    /// ```
    fn cols(&self) -> usize {
        self.cols
    }

    /// Gets a reference to the value at the specified `row` and `column`.  
    /// 
    /// # Returns
    /// Returns a reference to the value at `row`, `col`, otherwise `None` 
    /// if either `row` or `col` is outside of `Matrix`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    /// assert_eq!(matrix.get(2, 5).unwrap(), &17);
    /// assert_eq!(matrix.get(10, 2), None);
    /// ```
    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Gets a *mutable* reference to the value at specified `row` and `column`.  
    /// 
    /// # Returns
    /// Returns a mutable reference to the value at `row`, `col`, otherwise `None` 
    /// if either `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let mut matrix: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    ///
    /// *matrix.get_mut(0, 0).unwrap() = 5;
    /// assert_eq!(matrix.get(0, 0).unwrap(), &5);
    /// ```
    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Writes a value to a specified position in `Matrix`.
    /// 
    /// # Returns
    /// Returns `value` if successful, other `None` if either 
    /// `row` or `col` is outside of `Matrix`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let mut matrix: Matrix<usize> = Matrix::new(3, 3);
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    ///
    /// matrix.set(0, 0, 5);
    /// assert_eq!(matrix.get(0, 0).unwrap(), &5);
    /// ```
    fn set(&mut self, row: usize, col: usize, value: T) -> Option<&T> {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
            self.get(row, col)
        } else {
            None
        }
    }

    /// Gets an iterator of values from `row`.
    /// 
    /// # Returns
    /// Returns an iterator of the values in `row` if successful
    /// 
    /// # Panics
    /// Panics if `row` is outside the bounds of the `Matrix`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<i32> = Matrix::from_iter(3, 6, 0..);
    /// let result: Vec<i32> = matrix.get_row(1).unwrap().cloned().collect();
    /// 
    /// assert_eq!(result, [6, 7, 8, 9, 10, 11]);
    /// ```
    fn get_row(&self, row: usize) -> Result<RowIterator<T>, Box<dyn Error>> {
        assert!(row < self.rows);

        let iter = RowIterator {
            matrix: self,
            current_col: 0,
            current_row: row,
        };

        Ok(iter)
    }

    /// Gets an iterator of values from `col`.
    /// 
    /// # Returns
    /// Returns an iterator of the values in `col` if successful
    /// 
    /// # Panics
    /// Panics if `col` is outside the bounds of the `Matrix`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<i32> = Matrix::from_iter(3, 6, 0..);
    /// let result: Vec<i32> = matrix.get_col(1).unwrap().cloned().collect();
    /// 
    /// assert_eq!(result, [1, 7, 13]);
    /// ```
    fn get_col(&self, col: usize) -> Result<ColumnIterator<T>, Box<dyn Error>> {
        assert!(col < self.cols);

        let iter = ColumnIterator {
            matrix: self,
            current_col: col,
            current_row: 0,
        };

        Ok(iter)
    }

    /// Swaps the specified rows (`row_a` and `row_b`) of the `Matrix`.
    /// 
    /// # Returns
    /// Returns `()` if successful.
    /// 
    /// # Panics
    /// Panics if either `row_a` or `row_b` are outside `Matrix`.
    /// 
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    /// matrix.swap_rows(0, 1);
    /// 
    /// assert_eq!(matrix.get(0, 0).unwrap(), &2);
    /// assert_eq!(matrix.get(1, 1).unwrap(), &1);
    /// ```
    fn swap_rows(&mut self, row_a: usize, row_b: usize) -> Result<(), Box<dyn Error>> {
        assert!(row_a < self.rows);
        assert!(row_b < self.rows);

        for col in 0..self.cols {
            self.data.swap(row_a + col, (row_b * self.cols) + col);
        }

        Ok(())

    }

    /// Swaps the specified columns of the Matrix.
    /// 
    /// # Returns
    /// Returns `()` if successful.
    /// 
    /// # Panics
    /// Panics if either `row_a` or `row_b` are outside `Matrix`.
    /// 
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    /// matrix.swap_cols(0, 1);
    /// 
    /// assert_eq!(matrix.get(0, 0).unwrap(), &1);
    /// assert_eq!(matrix.get(1, 1).unwrap(), &2);
    /// ```
    fn swap_cols(&mut self, col_a: usize, col_b: usize) -> Result<(), Box<dyn Error>> {
        assert!(col_a < self.cols);
        assert!(col_b < self.cols);

        for row in 0..self.rows {
            self.data.swap((row * self.rows) + col_a, (row * self.rows) + col_b);
        }

        Ok(())

    }

    /// Transposes the `Matrix`.
    /// 
    /// Take an *m x n* Matrix *M* and return the *transpose* Matrix *M*<sup>T</sup>
    /// of size *n x m*.
    /// 
    /// # Returns
    /// Returns a new `Matrix<&T>' that is the transpose of the input `Matrix`. Values
    /// in the new matrix are references to the original matrix.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// let matrix: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// let transpose = matrix.transpose();
    ///
    /// assert_eq!(matrix.rows(), transpose.cols());
    /// assert_eq!(matrix.cols(), transpose.rows());
    ///     
    /// assert_eq!(matrix.get(0, 0).unwrap(), *transpose.get(0, 0).unwrap());
    /// assert_eq!(matrix.get(0, 2).unwrap(), *transpose.get(2, 0).unwrap());
    /// ```
    fn transpose(&self) -> Matrix<&T> {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for col in 0..self.cols {
                    let column_iter = self.get_col(col).unwrap();
                    data.extend(column_iter);
                }
                data
            },
        }
    }

    /// Applys a function to all values of the `Matrix`.  
    /// 
    /// Cells are provided as immutable references to the function,
    /// if you want to modify the cells, use `apply_mut`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// // Get the sum of all cells
    /// let matrix = Matrix::from_iter(3, 6, 0..);
    /// let mut sum = 0;
    /// matrix.apply(|n| sum += *n);
    /// 
    /// assert_eq!(sum, 153);
    /// ```
    fn apply<F: FnMut(&T)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    /// Applys a function to modify all values of the `Matrix`.
    /// 
    /// Matrix values are provided as mutable references to the function,
    /// and can therefore be modified.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::matrix::*;
    /// // Modify all cells with a function
    /// let mut matrix = Matrix::from_iter(3, 6, 0..);
    /// matrix.apply_mut(|n| *n *= 2);
    ///
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    /// assert_eq!(matrix.get(0, 1).unwrap(), &2);
    /// assert_eq!(matrix.get(0, 2).unwrap(), &4);
    /// ```
    fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}


/// Creates a new `Matrix<T>`
///
/// Note that the values are passed as an array, with a sub-array 
/// corresponding to each row.
///
/// # Example
/// ```
/// # #[macro_use] extern crate algorithms;
/// # use crate::algorithms::data_structures::matrix::*;
/// # fn main() {
/// let new_matrix = matrix![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
/// assert_eq!(new_matrix.get(1, 2).unwrap(), &6);
///
/// let other_matrix = matrix![[1], [2]];
/// assert_eq!(other_matrix.rows(), 2);
/// assert_eq!(other_matrix.cols(), 1);
/// # }
/// ```
#[macro_export]
macro_rules! matrix {
    () => {
        {
            let rows = 0;
            let cols = 0;
            let data = Vec::new();

            Matrix::from_vec(rows, cols, data)
        }
    };

    ([$($elem:expr),*]) => {
        {
            let data = vec![$($elem),*];
            let rows = 1;
            let cols = data.len();

            Matrix::from_vec(rows, cols, data)
        }
    };

    ($([$($elem:expr),*]),*) => {
        {
            let mut data = Vec::new();
            let mut rows = 0;
            $(
                let row = [$($elem),*];
                rows += 1;
                data.extend_from_slice(&row);
            )*
            let cols = if rows > 0 { row.len() } else { 0 };

            Matrix::from_vec(rows, cols, data)
        }
    };
}



pub struct ColumnIterator<'a, T> {
    matrix: &'a Matrix<T>,
    current_col: usize,
    current_row: usize,
}

impl<'a, T> Iterator for ColumnIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_col >= self.matrix.cols || self.current_row >= self.matrix.rows {
            None
        } else {
            let item = self.matrix.get(self.current_row, self.current_col);
            self.current_row += 1;

            item
        }
    }
}

pub struct RowIterator<'a, T> {
    matrix: &'a Matrix<T>,
    current_col: usize,
    current_row: usize,
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_col >= self.matrix.cols || self.current_row >= self.matrix.rows {
            None
        } else {
            let item = self.matrix.get(self.current_row, self.current_col);
            self.current_col += 1;

            item
        }
    }
}