use std::error::Error;


#[derive(Debug)]
pub struct Heap<T: Ord + Copy> {
    heap: Vec<T>
}

/// Defines methods expected on a heap data structure
pub trait IsHeap<T> {

    fn push(&mut self, value: T) -> Result<Option<T>, Box<dyn Error>>;

    fn pop(&mut self) -> Result<T, Box<dyn Error>>;

    fn get(&self) -> Result<T, Box<dyn Error>>;

    fn size(&self) -> usize;
}

impl<T> Heap<T> {
    /// Creates a new `Heap`.
    ///
    /// # Returns
    /// Returns an empty `Heap<T>`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::heap::*;
    /// let new_heap: Heap<isize> = Heap::new();
    /// 
    /// assert_eq!(new_heap.size(), 0);
    /// ```
    pub fn new() -> Heap<T> {
        Heap { heap: vec![] }
    }
}

impl<T> Default for Heap<T> {
    /// Initializes a `Default` `Heap`.
    ///
    /// # Returns
    /// Returns an empty `Heap<T>`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::heap::*;
    /// let new_heap: Heap<isize> = Heap::default();
    /// assert_eq!(new_heap.size(), 0);
    /// ```
    fn default() -> Heap<T> {
        Heap { heap: vec![] }
    }
}

impl<T> IsHeap<T> for Heap<T> {
    /// Adds a new element to the `Heap`.
    ///
    /// # Parameters
    /// - `value`: Value to add to the heap
    ///
    /// # Returns
    /// `None` as the element addition should always be successful.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::heap::*;
    /// let mut new_heap: Heap<isize> = Heap::new();
    /// assert_eq!(new_heap.add(42).unwrap(), None);
    /// assert_eq!(new_heap.size(), 1);
    /// ```
    fn push(&mut self, value: T) -> Result<Option<T>, Box<dyn Error>> {
        self.heap.push(value);
        Ok(None)
    }

    /// Removes the value from the top of the `Heap`.
    ///
    /// # Returns
    /// - `Ok(T)`: The value at the top of the heap
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the `Heap` is empty.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::heap::*;
    /// let mut new_heap: Heap<isize> = Heap::new();
    /// new_heap.add(42);
    /// assert_eq!(new_heap.remove().unwrap(), 42);
    /// assert_eq!(new_heap.size(), 0);
    /// ```
    fn pop(&mut self) -> Result<T, Box<dyn Error>> {
        if !self.stack.is_empty() {
            Ok(self.stack.pop().unwrap())
        } else {
            panic!("The heap is empty.")
        }
    }

    /// Gets a value in the `Heap`.
    ///
    /// # Returns
    /// - `T`: The specified value in the `Heap`.
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the `Heap` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::algorithms::data_structures::heap::*;
    /// let mut new_heap: Heap<isize> = Heap::new();
    /// new_heap.add(42);
    /// new_heap.add(43);
    /// assert_eq!(new_heap.read().unwrap(), 43);
    /// ```
    fn get(&self) -> Result<T, Box<dyn Error>> {
        match self.stack.last() {
            Some(val) => Ok(val.clone()),
            None => panic!("The heap is empty."),
        }
    }

    /// Gets the size of the `Heap`.
    ///
    /// # Returns
    /// Returns the number of elements in the `Heap`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::heap::*;
    /// let mut new_heap: Heap<isize> = Heap::new();
    /// assert_eq!(new_heap.size(), 0);
    /// 
    /// new_heap.add(42);
    /// assert_eq!(new_heap.size(), 1);
    /// ```
    fn size(&self) -> usize {
        self.heap.len()
    }
}

/// Creates a new `Heap<T>`
///
/// Delegates to the default heap initializer. 
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate algorithms;
/// # use crate::algorithms::data_structures::heap::*;
/// 
/// # fn main() {
/// let new_heap = heap![3, 4, 5];
/// assert_eq!(new_heap.read().unwrap(), 5);
///
/// let new_heap_empty: Heap<isize> = heap![];
/// assert_eq!(new_heap_empty.size(), 0);
/// # }
/// ```
#[macro_export]
macro_rules! heap {
    () => { Heap::new() };
    ($($x:expr),+) => {
        {
            let mut temp_q = Heap::default();
            $(
                let _ = temp_q.add($x);
            )*
            temp_q
        }
    };
}