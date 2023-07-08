use std::error::Error;

/// A simple stack (LIFO) implementation with a growable size and no capacity limit.
///
/// # Type parameters
/// - `T`: Generic type that implements the `Clone` trait.
///
/// # Examples
///
/// This example uses the `stack!` macro to add elements to the stack. Note
/// that the first element in the list of elements passed to the macro is
/// considered the 'oldest'.
///
/// ```
/// # #[macro_use] extern crate algorithms;
/// # use crate::algorithms::data_structures::stack::*;
/// # fn main() {
/// let mut stack = stack![3, 4, 5];
///
/// // Add an element
/// assert_eq!(stack.add(6).unwrap(), None);
///
/// // Remove some elements
/// assert_eq!(stack.remove().unwrap(), 3);
/// assert_eq!(stack.remove().unwrap(), 4);
///
/// // Peek at the next element scheduled for removal
/// assert_eq!(stack.read().unwrap(), 5);
///
/// // Check the stack size
/// assert_eq!(stack.size(), 2);
/// # }
/// ```
/// 
#[derive(Debug)]
pub struct Stack<T: Clone> {
    stack: Vec<T>
}

/// Defines methods expected on a stack data structure
pub trait IsStack<T: Clone> {
    /// Adds a new value to a stack.
    ///
    /// # Parameters
    /// - `val`: Value to add to the stack
    ///
    /// # Returns
    /// - `Ok(_)`: If the element add was successful.
    ///     - `Some(T)`: If adding an element resulted in the removal of an
    ///         existing one (in the case of a circular buffer, for instance)
    ///     - `None`: Adding an element did not return any value
    /// - `Error`: If the element add was unsuccessful
    ///
    /// # Errors
    /// Attempting to add an element to a full stack that does not allow for
    /// overflow will return an error.
    fn add(&mut self, value: T) -> Result<Option<T>, Box<dyn Error>>;

    /// Removes an element from the stack and returns it.
    ///
    /// For stacks with default values, removing an element will add a new
    /// default value into the stack.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest value in the stack
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    fn remove(&mut self) -> Result<T, Box<dyn Error>>;

    /// Reads the end of the stack.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest value in the stack
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the stack is empty.
    fn read(&self) -> Result<T, Box<dyn Error>>;

    /// Gets the size of the stack.
    ///
    /// # Returns
    /// The number of elements in the stack. Note, this *includes* default
    /// values when specified, which means that the `size` of a stack with
    /// default values should always be equal to its `capacity`
    fn size(&self) -> usize;
}

impl<T: Clone> Stack<T> {
    /// Creates a new simple `stack`.
    ///
    /// # Returns
    /// Returns an empty `Stack<T>`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::stack::*;
    /// let stack: Stack<isize> = Stack::new();
    /// 
    /// assert_eq!(stack.size(), 0);
    /// ```
    pub fn new() -> Stack<T> {
        Stack { stack: vec![] }
    }
}

impl<T: Clone> Default for Stack<T> {
    /// Initializes a `Default` `stack`.
    ///
    /// # Returns
    /// Returns an empty `Stack<T>`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::stack::*;
    /// let q: Stack<isize> = Stack::default();
    /// assert_eq!(q.size(), 0);
    /// ```
    fn default() -> Stack<T> {
        Stack { stack: vec![] }
    }
}

impl<T: Clone> IsStack<T> for Stack<T> {
    /// Adds a new element to the end of the `stack`.
    ///
    /// # Parameters
    /// - `value`: Value to add to the stack
    ///
    /// # Returns
    /// `None` as the element addition should always be successful.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::stack::*;
    /// let mut stack: Stack<isize> = Stack::new();
    /// assert_eq!(stack.add(42).unwrap(), None);
    /// assert_eq!(stack.size(), 1);
    /// ```
    fn add(&mut self, value: T) -> Result<Option<T>, Box<dyn Error>> {
        self.stack.push(value);
        Ok(None)
    }

    /// Removes the last value from the `stack` and returns it.
    ///
    /// # Returns
    /// - `Ok(T)`: The last value in the stack
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the `stack` is empty.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::stack::*;
    /// let mut q: Stack<isize> = Stack::new();
    /// q.add(42);
    /// assert_eq!(q.remove().unwrap(), 42);
    /// assert_eq!(q.size(), 0);
    /// ```
    fn remove(&mut self) -> Result<T, Box<dyn Error>> {
        if !self.stack.is_empty() {
            Ok(self.stack.pop().unwrap())
        } else {
            panic!("The stack is empty")
        }
    }

    /// Reads the last value in the `stack`.
    ///
    /// # Returns
    /// - `T`: The oldest value in the stack
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the `stack` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::algorithms::data_structures::stack::*;
    /// let mut stack: Stack<isize> = Stack::new();
    /// stack.add(42);
    /// stack.add(43);
    /// assert_eq!(stack.read().unwrap(), 43);
    /// ```
    fn read(&self) -> Result<T, Box<dyn Error>> {
        match self.stack.last() {
            Some(val) => Ok(val.clone()),
            None => panic!("The stack is empty"),
        }
    }

    /// Gets the size of the `stack`.
    ///
    /// # Returns
    /// Returns the number of elements in the stack.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::stack::*;
    /// let mut q: Stack<isize> = Stack::new();
    /// assert_eq!(q.size(), 0);
    /// 
    /// q.add(42);
    /// assert_eq!(q.size(), 1);
    /// ```
    fn size(&self) -> usize {
        self.stack.len()
    }
}

/// Creates a new `Stack<T>`
///
/// Delegates to the default stack initializer. Note that the values are
/// added to the stack from left to right, therefore the first element in the
/// list of parameters passed to the macro is considered the 'oldest' element
/// in the stack.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate algorithms;
/// # use crate::algorithms::data_structures::stack::*;
/// 
/// # fn main() {
/// let q = stack![3, 4, 5];
/// assert_eq!(q.read().unwrap(), 5);
///
/// let q_empty: Stack<isize> = stack![];
/// assert_eq!(q_empty.size(), 0);
/// # }
/// ```
#[macro_export]
macro_rules! stack {
    () => { Stack::new() };
    ($($x:expr),+) => {
        {
            let mut temp_q = Stack::default();
            $(
                let _ = temp_q.add($x);
            )*
            temp_q
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack: Stack<isize> = Stack::new();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_add() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.add(42).unwrap(), None);
        assert_eq!(stack.size(), 1);
    }

    #[test]
    fn test_remove() {
        let mut stack: Stack<isize> = Stack::new();
        stack.add(42);
        assert_eq!(stack.remove().unwrap(), 42);
        assert_eq!(stack.size(), 0);   
    }

    #[test]
    fn test_read() {
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
}