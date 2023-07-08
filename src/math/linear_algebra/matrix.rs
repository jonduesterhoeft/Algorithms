use std::slice::Iter;
use num_traits::Num;

/// A simple *m x n* Matrix, with *m* `rows` and *n* `cols`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> Matrix<T> 
{    
    /// Constructs an *m x n* Matrix, with *m* `rows` and *n* `cols` 
    /// with initialized to zero. 
    ///  
    /// Use `Matrix::from_iter` to build from an iterator instead.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are zero or less.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<i32> = new(2, 4);
    /// 
    /// assert_eq!(matrix.get(1, 1).unwrap(), &0);
    /// ```
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Default + Num
    {
        Matrix::from_iter(rows, cols, (0..).map(|_| num_traits::zero()))
    }

    /// Constructs an *m x n* Matrix, with *m* `rows` and *n* `cols` 
    /// where data is populated from an iterator.  
    /// 
    /// The matrix values are set row by row.  
    /// 
    /// Only `rows * cols` values will be consumed from the iterator.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are zero or less.
    /// Panics if the iterator does not have at least `rows * cols` values.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), &0);
    /// assert_eq!(mat.get(0, 1).unwrap(), &1);
    /// assert_eq!(mat.get(1, 0).unwrap(), &2);
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

    /// Constructs a *m x m* identity matrix.
    /// 
    /// The matrix must be square so only one dimension is needed.
    ///
    /// # Panics
    /// Panics if `size` is zero or less.
    /// 
    /// # Examples
    /// ```
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

    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<usize> = Matrix::new(3, 6);
    ///
    /// assert_eq!(matrix.rows(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<usize> = Matrix::new(3, 6);
    ///
    /// assert_eq!(matrix.cols(), 6);
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Try to get a reference to the value at specified `row` and `column`.  
    /// 
    /// Returns `None` if either `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<usize> = Matrix::new(3, 6, 0..);
    ///
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    /// assert_eq!(matrix.get(2, 5).unwrap(), &17);
    /// assert_eq!(matrix.get(10, 2), None);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Try to get a mutable reference to the value at specified `row` and `column`.  
    /// 
    /// Returns `None` if either `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// let mut matrix: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    ///
    /// *matrix.get_mut(0, 0).unwrap() = 5;
    /// assert_eq!(matrix.get(0, 0).unwrap(), &5);
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Attempt to write a value to a specified position in the matrix.
    ///   
    /// Returns `None` if either `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// let mut matrix: Matrix<usize> = Matrix::new(3, 3);
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    ///
    /// matrix.set(0, 0, 5);
    /// assert_eq!(matrix.get(0, 0).unwrap(), &5);
    /// ```
    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
            true
        } else {
            false
        }
    }

    /// Try to get an iterator of values from `row`.
    /// 
    /// Returns `None` if `row` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(matrix.get_row(1).unwrap(), vec![6, 7, 8, 9, 10, 11]);
    ///
    /// assert!(matrix.get_row(5).is_err());
    /// ```
    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows {
            Some((0..self.cols).map(move |col| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    /// Try to get an iterator of values from `column`.  
    /// 
    /// Returns `None` if `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// let mat: Matrix<usize> = Matrix::new(3, 6, 0..);
    ///
    /// assert_eq!(mat.get_col(1).unwrap(), vec![1, 7, 13]);
    ///
    /// assert!(mat.get_col(10).is_err());
    /// ```
    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols {
            Some((0..self.rows).map(move |row| self.get(row, col).unwrap()))
        } else {
            None
        }
    }

    /// Swaps the specified rows of the Matrix.
    /// 
    /// # Panics
    /// Panics if either `row_a` or `row_b` are outside the Matrix.
    /// 
    /// # Examples
    /// ```
    /// let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    /// matrix.swap_rows(0, 1);
    /// 
    /// assert_eq!(matrix.get(0, 0).unwrap(), &2);
    /// assert_eq!(matrix.get(1, 1).unwrap(), &1);
    /// ```
    pub fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        assert!(row_a < self.rows);
        assert!(row_b < self.rows);

        for col in 0..self.cols {
            self.data.swap(row_a + col, (row_b * self.cols) + col);
        }

    }

    /// Swaps the specified columns of the Matrix.
    /// 
    /// # Panics
    /// Panics if either `col_a` or `col_b` are outside the Matrix.
    /// 
    /// # Examples
    /// ```
    /// let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    /// matrix.swap_cols(0, 1);
    /// 
    /// assert_eq!(matrix.get(0, 0).unwrap(), &1);
    /// assert_eq!(matrix.get(1, 1).unwrap(), &2);
    /// ```
    pub fn swap_cols(&mut self, col_a: usize, col_b: usize) {
        assert!(col_a < self.cols);
        assert!(col_b < self.cols);

        for row in 0..self.rows {
            self.data.swap((row * self.rows) + col_a, (row * self.rows) + col_b);
        }

    }

    /// Take an *m x n* Matrix *M* and return the *transpose* Matrix *M*<sup>T</sup>.
    ///
    /// # Examples
    /// ```
    /// let matrix: Matrix<usize> = Matrix::new(3, 6, 0..);
    /// let transpose = matrix.transpose();
    ///
    /// assert_eq!(matrix.rows(), transpose.cols());
    /// assert_eq!(matrix.cols(), transpose.rows());
    ///     
    /// assert_eq!(matrix.get(0, 0).unwrap(), transpose.get(0, 0).unwrap());
    /// assert_eq!(matrix.get(0, 2).unwrap(), transpose.get(2, 0).unwrap());
    /// ```
    pub fn transpose(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for col in 0..self.cols {
                    for val in self.get_col(col).unwrap() {
                        data.push(val.clone());
                    }
                }
                data
            },
        }
    }

    /// Apply a function to all values of the matrix.  
    /// 
    /// Cells are provided as immutable references to the function,
    /// if you want to modify the cells, use `apply_mut`.
    ///
    /// # Examples
    /// ```
    /// // Get the sum of all cells
    /// let matrix = Matrix::from_iter(3, 6, 0..);
    /// let mut sum = 0;
    /// matrix.apply(|n| sum += *n);
    /// 
    /// assert_eq!(sum, 153);
    /// ```
    pub fn apply<F: FnMut(&T)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    /// Pass a function to modify all values of the Matrix.
    /// 
    /// Matrix values are provided as mutable references to the function,
    /// and can therefore be modified.
    ///
    /// # Examples
    /// ```
    /// // Modify all cells with a function
    /// let mut matrix = Matrix::from_iter(3, 6, 0..);
    /// matrix.apply_mut(|n| n *= 2);
    ///
    /// assert_eq!(matrix.get(0, 0).unwrap(), &0);
    /// assert_eq!(matrix.get(0, 1).unwrap(), &2);
    /// assert_eq!(matrix.get(0, 2).unwrap(), &4);
    /// ```
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_matrix() {
        let matrix: Matrix<i32> = Matrix::new(2, 4);
        assert_eq!(matrix.get(1, 1).unwrap(), &0);
    }

    #[test]
    fn test_new_from_iter() {
        let matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
    
        assert_eq!(matrix.get(0, 0).unwrap(), &0);
        assert_eq!(matrix.get(0, 1).unwrap(), &1);
        assert_eq!(matrix.get(1, 0).unwrap(), &2);
    }

    #[test]
    fn test_identity() {
        let identity: Matrix<i32> = Matrix::identity(3);
        assert_eq!(identity.get(0, 0).unwrap(), &1);
        assert_eq!(identity.get(0, 1).unwrap(), &0);
        assert_eq!(identity.get(2, 2).unwrap(), &1);
    }

    #[test]
    fn test_rows() {
        let matrix: Matrix<usize> = Matrix::new(3, 6);
        assert_eq!(matrix.rows(), 3);
    }

    #[test]
    fn test_cols() {
        let matrix: Matrix<usize> = Matrix::new(3, 6);
        assert_eq!(matrix.cols(), 6);
    }

    #[test]
    fn test_get_value() {
        let matrix = Matrix::from_iter(3, 6, 0..);
        assert_eq!(matrix.get(0, 0).unwrap(), &0);
        assert_eq!(matrix.get(2, 5).unwrap(), &17);
        assert_eq!(matrix.get(10, 2), None);
    }

    #[test]
    fn test_get_mut_value() {
        let mut matrix: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
        assert_eq!(matrix.get(0, 0).unwrap(), &0);

        *matrix.get_mut(0, 0).unwrap() = 5;
        assert_eq!(matrix.get(0, 0).unwrap(), &5);
    }

    #[test]
    fn test_set_value() {
        let mut matrix: Matrix<usize> = Matrix::new(3, 3);
        assert_eq!(matrix.get(0, 0).unwrap(), &0);
        
        matrix.set(0, 0, 5);
        assert_eq!(matrix.get(0, 0).unwrap(), &5);
    }

    #[test]
    fn test_get_row() {
        let matrix = Matrix::from_iter(3, 6, 0..);
        let result: Vec<i32> = matrix.get_row(1).unwrap().cloned().collect();
        assert_eq!(result, vec![6, 7, 8, 9, 10, 11]);
        assert!(matrix.get_row(10).is_none());
    }

    #[test]
    fn test_get_col() {
        let matrix = Matrix::from_iter(3, 6, 0..);
        let result: Vec<i32> = matrix.get_col(1).unwrap().cloned().collect();
        assert_eq!(result, vec![1, 7, 13]);
        assert!(matrix.get_col(10).is_none());
    }

    #[test]
    fn test_transpose() {
        let matrix: Matrix<usize> = Matrix::from_iter(2, 3, 0..);
        let transpose = matrix.transpose();
        
        assert_eq!(matrix.rows(), transpose.cols());
        assert_eq!(matrix.cols(), transpose.rows());
        
        assert_eq!(matrix.get(0, 0).unwrap(), transpose.get(0, 0).unwrap());
        assert_eq!(matrix.get(0, 2).unwrap(), transpose.get(2, 0).unwrap());
    }


    #[test]
    fn test_swap_rows() {
        let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
        matrix.swap_rows(0, 1);
        assert_eq!(matrix.get(0, 0).unwrap(), &2);
        assert_eq!(matrix.get(0, 1).unwrap(), &3);
        assert_eq!(matrix.get(1, 0).unwrap(), &0);
        assert_eq!(matrix.get(1, 1).unwrap(), &1);
    }

    #[test]
    #[should_panic]
    fn test_swap_rows_out_of_bounds() {
        let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
        matrix.swap_rows(0, 3);
    }


    #[test]
    fn test_swap_cols() {
        let mut matrix: Matrix<usize> = Matrix::from_iter(3, 3, 0..);
        matrix.swap_cols(0, 2);
        assert_eq!(matrix.get(0, 0).unwrap(), &2);
        assert_eq!(matrix.get(1, 1).unwrap(), &4);
        assert_eq!(matrix.get(2, 2).unwrap(), &6);
    }

    #[test]
    #[should_panic]
    fn test_swap_cols_out_of_bounds() {
        let mut matrix: Matrix<usize> = Matrix::from_iter(2, 2, 0..);
        matrix.swap_cols(0, 3);
    }

    #[test]
    fn test_apply_function() {
        let matrix = Matrix::from_iter(3, 6, 0..);
        let mut sum = 0;
        matrix.apply(|n| sum += *n);
        assert_eq!(sum, 153);
    }

    #[test]
    fn test_apply_mut_function() {
        let mut matrix = Matrix::from_iter(3, 6, 0..);
        matrix.apply_mut(|n| *n *= 2);
        assert_eq!(matrix.get(0, 4).unwrap(), &8);
    }

}
