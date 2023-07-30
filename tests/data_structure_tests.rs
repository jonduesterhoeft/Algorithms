use algorithms::data_structures::{stack::*, queue::*, matrix::*};
use algorithms::matrix;



// Stack Tests
#[test]
fn test_stack_new_stack() {
    let stack: Stack<isize> = Stack::new();
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_stack_add() {
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.add(42).unwrap(), None);
    assert_eq!(stack.size(), 1);
}

#[test]
fn test_stack_remove() {
    let mut stack: Stack<isize> = Stack::new();
    stack.add(42);
    assert_eq!(stack.remove().unwrap(), 42);
    assert_eq!(stack.size(), 0);   
}

#[test]
fn test_stack_read() {
    let mut stack: Stack<isize> = Stack::new();
    stack.add(42);
    assert_eq!(stack.read().unwrap(), 42);
}

#[test]
#[should_panic]
fn test_read_empty() {
    let mut stack: Stack<isize> = Stack::new();
    stack.read();
}


// Queue Tests
#[test]
fn test_queue_new_queue() {
    let queue: Queue<isize> = Queue::new();
    assert_eq!(queue.size(), 0);
}

#[test]
fn test_queue_add() {
    let mut queue: Queue<i32> = Queue::new();
    assert_eq!(queue.add(42).unwrap(), None);
    assert_eq!(queue.size(), 1);
}

#[test]
fn test_queue_remove() {
    let mut q: Queue<isize> = Queue::new();
    q.add(42);
    assert_eq!(q.remove().unwrap(), 42);
    assert_eq!(q.size(), 0);   
}

#[test]
fn test_queue_read() {
    let mut queue: Queue<isize> = Queue::new();
    queue.add(42);
    assert_eq!(queue.read().unwrap(), 42);
}

#[test]
#[should_panic]
fn test_queue_read_empty() {
    let mut queue: Queue<isize> = Queue::new();
    queue.read();
}


// Matrix Tests
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
    assert_eq!(result, vec![6, 7, 8 ,9 , 10 ,11]);
}

#[test]
#[should_panic]
fn test_get_row_out_of_bounds() {
    let matrix = Matrix::from_iter(3, 6, 0..);
    matrix.get_row(10);
}

#[test]
fn test_get_col() {
    let matrix = Matrix::from_iter(3, 6, 0..);
    let result: Vec<i32> = matrix.get_col(1).unwrap().cloned().collect();
    assert_eq!(result, vec![1, 7, 13]);
}

#[test]
#[should_panic]
fn test_get_col_out_of_bounds() {
    let matrix = Matrix::from_iter(3, 6, 0..);
    matrix.get_col(10);
}

#[test]
fn test_transpose() {
    let matrix: Matrix<usize> = Matrix::from_iter(2, 3, 0..);
    let transpose = matrix.transpose();
    
    assert_eq!(matrix.rows(), transpose.cols());
    assert_eq!(matrix.cols(), transpose.rows());
    
    assert_eq!(matrix.get(0, 0).unwrap(), *transpose.get(0, 0).unwrap());
    assert_eq!(matrix.get(0, 2).unwrap(), *transpose.get(2, 0).unwrap());
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

#[test]
fn test_macro() {
    let new_matrix = matrix![[1, 2], [3, 4]];
    assert_eq!(new_matrix.get(0, 1).unwrap(), &2);
}