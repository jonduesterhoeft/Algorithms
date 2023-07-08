use std::error::Error;

/// A simple queue (FIFO) implementation with a growable size and no capacity limit.
///
/// # Type parameters
/// - `T`: Generic type that implements the `Clone` trait.
///
/// # Examples
///
/// This example uses the `queue!` macro to add elements to the queue. Note
/// that the first element in the list of elements passed to the macro is
/// considered the 'oldest'.
///
/// ```
/// # #[macro_use] extern crate algorithms;
/// # use crate::algorithms::data_structures::queue::*;
/// # fn main() {
/// let mut queue = queue![3isize, 4, 5];
///
/// // Add an element
/// assert_eq!(queue.add(6).unwrap(), None);
///
/// // Remove some elements
/// assert_eq!(queue.remove().unwrap(), 3);
/// assert_eq!(queue.remove().unwrap(), 4);
///
/// // Peek at the next element scheduled for removal
/// assert_eq!(queue.read().unwrap(), 5);
///
/// // Check the queue size
/// assert_eq!(queue.size(), 2);
/// # }
/// ```
/// 
#[derive(Debug)]
pub struct Queue<T: Clone> {
    queue: Vec<T>
}

/// Defines methods expected on a queue data structure
pub trait IsQueue<T: Clone> {
    /// Adds a new value to a queue
    ///
    /// # Parameters
    /// - `val`: Value to add to the queue
    ///
    /// # Returns
    /// - `Ok(_)`: If the element add was successful.
    ///     - `Some(T)`: If adding an element resulted in the removal of an
    ///         existing one (in the case of a circular buffer, for instance)
    ///     - `None`: Adding an element did not return any value
    /// - `Error`: If the element add was unsuccessful
    ///
    /// # Errors
    /// Attempting to add an element to a full queue that does not allow for
    /// overflow will return an error.
    fn add(&mut self, value: T) -> Result<Option<T>, Box<dyn Error>>;

    /// Removes an element from the queue and returns it
    ///
    /// For queues with default values, removing an element will add a new
    /// default value into the queue.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest value in the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the queue is empty.
    fn remove(&mut self) -> Result<T, Box<dyn Error>>;

    /// Reads the head of the queue.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest value in the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the queue is empty.
    fn read(&self) -> Result<T, Box<dyn Error>>;

    /// Gets the size of the queue.
    ///
    /// # Returns
    /// The number of elements in the queue. Note, this *includes* default
    /// values when specified, which means that the `size` of a queue with
    /// default values should always be equal to its `capacity`
    fn size(&self) -> usize;
}

impl<T: Clone> Queue<T> {
    /// Creates a new simple `Queue`.
    ///
    /// # Returns
    /// Returns an empty `Queue<T>`.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::queue::*;
    /// let queue: Queue<isize> = Queue::new();
    /// 
    /// assert_eq!(queue.size(), 0);
    /// ```
    pub fn new() -> Queue<T>
    {
        Queue { queue: vec![] }
    }
}

impl<T: Clone> Default for Queue<T> {
    /// Initializes a `Default` `Queue`.
    ///
    /// # Returns
    /// Returns an empty `Queue<T>`
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::queue::*;
    /// let queue: Queue<isize> = Queue::default();
    /// assert_eq!(queue.size(), 0);
    /// ```
    fn default() -> Queue<T> {
        Queue { queue: vec![] }
    }
}

impl<T: Clone> IsQueue<T> for Queue<T> {
    /// Adds a new element to the end of the `Queue`.
    ///
    /// # Parameters
    /// - `value`: Value to add to the queue
    ///
    /// # Returns
    /// `None` as the element addition should always be successful.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::queue::*;
    /// let mut queue: Queue<isize> = Queue::new();
    /// assert_eq!(queue.add(42).unwrap(), None);
    /// assert_eq!(queue.size(), 1);
    /// ```
    fn add(&mut self, value: T) -> Result<Option<T>, Box<dyn Error>> {
        self.queue.push(value);
        Ok(None)
    }

    /// Removes the oldest value from the `Queue` and returns it.
    ///
    /// # Returns
    /// - `Ok(T)`: The oldest value in the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the `Queue` is empty.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::queue::*;
    /// let mut q: Queue<isize> = Queue::new();
    /// q.add(42);
    /// assert_eq!(q.remove().unwrap(), 42);
    /// assert_eq!(q.size(), 0);
    /// ```
    fn remove(&mut self) -> Result<T, Box<dyn Error>> {
        if !self.queue.is_empty() {
            Ok(self.queue.remove(0))
        } else {
            panic!("The queue is empty")
        }
    }

    /// Reads the oldest value in the `Queue`.
    ///
    /// # Returns
    /// - `T`: The oldest value in the queue
    /// - `Error`
    ///
    /// # Errors
    /// Returns an error if the `Queue` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::algorithms::data_structures::queue::*;
    /// let mut queue: Queue<isize> = Queue::new();
    /// queue.add(42);
    /// assert_eq!(queue.read().unwrap(), 42);
    /// ```
    fn read(&self) -> Result<T, Box<dyn Error>> {
        match self.queue.first() {
            Some(val) => Ok(val.clone()),
            None => panic!("The queue is empty"),
        }
    }

    /// Gets the size of the `Queue`.
    ///
    /// # Returns
    /// Returns the number of elements in the queue.
    ///
    /// # Examples
    /// ```
    /// # use crate::algorithms::data_structures::queue::*;
    /// let mut q: Queue<isize> = Queue::new();
    /// assert_eq!(q.size(), 0);
    /// 
    /// q.add(42);
    /// assert_eq!(q.size(), 1);
    /// ```
    fn size(&self) -> usize {
        self.queue.len()
    }
}

/// Creates a new `Queue<T>`
///
/// Delegates to the default queue initializer. Note that the values are
/// added to the queue from left to right, therefore the first element in the
/// list of parameters passed to the macro is considered the 'oldest' element
/// in the queue.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate algorithms;
/// # use crate::algorithms::data_structures::queue::*;
/// 
/// # fn main() {
/// let q = queue![3, 4, 5];
/// assert_eq!(q.read().unwrap(), 3);
///
/// let q_empty: Queue<isize> = queue![];
/// assert_eq!(q_empty.size(), 0);
/// # }
/// ```
#[macro_export]
macro_rules! queue {
    () => { Queue::new() };
    ($($x:expr),+) => {
        {
            let mut temp_q = Queue::default();
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
    fn test_new_queue() {
        let queue: Queue<isize> = Queue::new();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_add() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.add(42).unwrap(), None);
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn test_remove() {
        let mut q: Queue<isize> = Queue::new();
        q.add(42);
        assert_eq!(q.remove().unwrap(), 42);
        assert_eq!(q.size(), 0);   
    }

    #[test]
    fn test_read() {
        let mut queue: Queue<isize> = Queue::new();
        queue.add(42);
        assert_eq!(queue.read().unwrap(), 42);
    }

    #[test]
    #[should_panic]
    fn test_read_empty() {
        let mut queue: Queue<isize> = Queue::new();
        queue.read();
    }
}