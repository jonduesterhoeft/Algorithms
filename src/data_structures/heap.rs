use std::cmp::PartialOrd;

/// Represents a binary heap data structure.
///
/// The `BinaryHeap` enum can contain either a `MinHeap` or `MaxHeap` variant.
/// Both variants implement common methods to create, build, and maintain the heap.
/// Use the associated functions provided by the enum to create instances of MinHeap and MaxHeap.
///
/// # Examples
///
/// Creating a new MinHeap:
/// ```
/// # use crate::algorithms::data_structures::heap::BinaryHeap;
/// // Specify the type when creating an empty heap.
/// let min_heap: BinaryHeap<i32> = BinaryHeap::new_min();
/// ```
///
/// Creating a new MaxHeap from existing data:
/// ```
/// # use crate::algorithms::data_structures::heap::BinaryHeap;
/// let existing_data = vec![4, 10, 3, 5, 1];
/// let max_heap = BinaryHeap::from_data_max(existing_data);
/// ```
pub enum BinaryHeap<T> 
where
    T: PartialOrd
{
    Min(MinHeap<T>),
    Max(MaxHeap<T>)
}

/// Represents a Min-Heap data structure.
///
/// The `MinHeap` struct provides methods to create a new empty MinHeap or
/// build a MinHeap from an existing data vector. It also has functions to
/// maintain the MinHeap property.
///
/// # Examples
///
/// Creating a new empty MinHeap:
/// ```
/// # use crate::algorithms::data_structures::heap::MinHeap;
/// // Specify the type when creating an empty heap.
/// let min_heap: MinHeap<i32> = MinHeap::new();
/// ```
///
/// Creating a new MinHeap from existing data:
/// ```
/// # use crate::algorithms::data_structures::heap::MinHeap;
/// let existing_data = vec![4, 10, 3, 5, 1];
/// let min_heap = MinHeap::from_data(existing_data);
/// ```
pub struct MinHeap<T>
where 
    T: PartialOrd
{
    data: Vec<T>,
}

/// Represents a Max-Heap data structure.
///
/// The `MaxHeap` struct provides methods to create a new empty MaxHeap or
/// build a MaxHeap from an existing data vector. It also has functions to
/// maintain the MaxHeap property.
///
/// # Examples
///
/// Creating a new empty MaxHeap:
/// ```
/// # use crate::algorithms::data_structures::heap::MaxHeap;
/// // Specify the type when creating an empty heap.
/// let max_heap: MaxHeap<i32> = MaxHeap::new();
/// ```
///
/// Creating a new MaxHeap from existing data:
/// ```
/// # use crate::algorithms::data_structures::heap::MaxHeap;
/// let existing_data = vec![4, 10, 3, 5, 1];
/// let max_heap = MaxHeap::from_data(existing_data);
/// ```
pub struct MaxHeap<T>
where 
    T: PartialOrd
{
    data: Vec<T>,
}

impl<T> BinaryHeap<T> 
where
    T: PartialOrd
{
    /// Creates an empty MinHeap.
    ///
    /// This function creates a new empty MinHeap and returns it as `MinHeap<T>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::{BinaryHeap, MinHeap};
    /// let min_heap: BinaryHeap<i32> = BinaryHeap::new_min();
    /// ```
    pub fn new_min() -> Self {
        BinaryHeap::Min(MinHeap::new())
    }

    /// Creates a new empty MaxHeap.
    ///
    /// This function creates a new empty MaxHeap and returns it as `MaxHeap<T>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::{BinaryHeap, MaxHeap};
    /// let max_heap: BinaryHeap<i32> = BinaryHeap::new_max();
    /// ```
    pub fn new_max() -> Self {
        BinaryHeap::Max(MaxHeap::new())
    }

    /// Creates a new MinHeap from an existing data vector.
    ///
    /// This function takes a `Vec<T>` as input and creates a new MinHeap from it.
    /// The MinHeap property is enforced during the creation process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::{BinaryHeap, MinHeap};
    /// let existing_data = vec![4, 10, 3, 5, 1];
    /// let min_heap: BinaryHeap<i32> = BinaryHeap::from_data_min(existing_data);
    /// ```
    pub fn from_data_min(data: Vec<T>) -> Self {
        BinaryHeap::Min(MinHeap::from_data(data))
    }

    /// Creates a new MaxHeap from an existing data vector.
    ///
    /// This function takes a `Vec<T>` as input and creates a new MaxHeap from it.
    /// The MaxHeap property is enforced during the creation process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::{BinaryHeap, MaxHeap};
    /// let existing_data = vec![4, 10, 3, 5, 1];
    /// let max_heap: BinaryHeap<i32> = BinaryHeap::from_data_max(existing_data);
    /// ```
    pub fn from_data_max(data: Vec<T>) -> Self {
        BinaryHeap::Max(MaxHeap::from_data(data))
    }

    /// Returns the number of elements in the binary heap.
    ///
    /// This method returns the number of elements in the binary heap.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::BinaryHeap;
    /// let min_heap: BinaryHeap<i32> = BinaryHeap::new_min();
    /// assert_eq!(min_heap.len(), 0);
    ///
    /// let existing_data = vec![4, 10, 3, 5, 1];
    /// let min_heap_from_data: BinaryHeap<i32> = BinaryHeap::from_data_min(existing_data);
    /// assert_eq!(min_heap_from_data.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        match self {
            BinaryHeap::Min(heap) => heap.data.len(),
            BinaryHeap::Max(heap) => heap.data.len(),
        }
    }
}


impl<T> MinHeap<T> 
where
    T: PartialOrd
{
    /// Creates a new empty MinHeap.
    ///
    /// This function creates a new empty MinHeap and returns it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::MinHeap;
    /// let min_heap: MinHeap<i32> = MinHeap::new();
    /// ```
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    /// Creates a new MinHeap from an existing data vector.
    ///
    /// This function takes a `Vec<T>` as input and creates a new MinHeap from it.
    /// The MinHeap property is enforced during the creation process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::MinHeap;
    /// let existing_data = vec![4, 10, 3, 5, 1];
    /// let min_heap_from_data: MinHeap<i32> = MinHeap::from_data(existing_data);
    /// ```
    pub fn from_data(data: Vec<T>) -> Self {
        let mut new_heap = MinHeap { data };
        new_heap.build_heap();
        new_heap
    }

    /// Organizes the data into a min-heap.
    ///
    /// This function takes the data in the `Vec<T>` and organizes it into a min-heap,
    /// satisfying the min-heap property. It starts from the last non-leaf node and
    /// performs the min-heapify operation on each node in reverse order, ensuring that
    /// the entire binary tree satisfies the min-heap property.
    fn build_heap(&mut self) {
        let heap_size = self.data.len();
        for i in (0..=(heap_size / 2)).rev() {
            self.min_heapify(&i);
        }
    }

    /// Maintains the min-heap property.
    ///
    /// This function ensures that the min-heap property is satisfied for a given node
    /// and its left and right subtrees. If the value at the given node is greater than
    /// either of its children, it swaps the node's value with the smallest child and
    /// continues recursively until the entire binary tree satisfies the min-heap property.
    fn min_heapify(&mut self, i: &usize) {
        let l = left(i);
        let r = right(i);
        let mut smallest: usize;

        if l < self.data.len() && self.data[l] < self.data[*i] {
            smallest = l;
        } else {
            smallest = *i;
        }

        if r < self.data.len() && self.data[r] < self.data[smallest] {
            smallest = r;
        }

        if smallest != *i {
            self.data.swap(*i, smallest);
            self.min_heapify(&smallest);
        }
    }
}


impl<T> MaxHeap<T> 
where
    T: PartialOrd
{
    /// Creates a new empty MaxHeap.
    ///
    /// This function creates a new empty MaxHeap and returns it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::MaxHeap;
    /// let max_heap: MaxHeap<i32> = MaxHeap::new();
    /// ```
    pub fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    /// Creates a new MaxHeap from an existing data vector.
    ///
    /// This function takes a `Vec<T>` as input and creates a new MaxHeap from it.
    /// The MaxHeap property is enforced during the creation process.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use crate::algorithms::data_structures::heap::MaxHeap;
    /// let existing_data = vec![4, 10, 3, 5, 1];
    /// let max_heap_from_data: MaxHeap<i32> = MaxHeap::from_data(existing_data);
    /// ```
    pub fn from_data(data: Vec<T>) -> Self {
        let mut new_heap = MaxHeap { data };
        new_heap.build_heap();
        new_heap
    }

    /// Organizes the data into a max-heap.
    ///
    /// This function takes the data in the `Vec<T>` and organizes it into a max-heap,
    /// satisfying the max-heap property. It starts from the last non-leaf node and
    /// performs the max-heapify operation on each node in reverse order, ensuring that
    /// the entire binary tree satisfies the max-heap property.
    fn build_heap(&mut self) {
        let heap_size = self.data.len();
        for i in (0..=(heap_size / 2)).rev() {
            self.max_heapify(&i);
        }
    }

    /// Maintains the max-heap property.
    ///
    /// This function ensures that the max-heap property is satisfied for a given node
    /// and its left and right subtrees. If the value at the given node is smaller than
    /// either of its children, it swaps the node's value with the largest child and
    /// continues recursively until the entire binary tree satisfies the max-heap property.
    fn max_heapify(&mut self, i: &usize) {
        let l = left(i);
        let r = right(i);
        let mut largest: usize;

        if l < self.data.len() && self.data[l] > self.data[*i] {
            largest = l;
        } else {
            largest = *i;
        }

        if r < self.data.len() && self.data[r] > self.data[largest] {
            largest = r;
        }

        if largest != *i {
            self.data.swap(*i, largest);
            self.max_heapify(&largest);
        }
    }
}





fn _parent(i: &usize) -> usize {
    (i - 1) / 2
}

fn left(i: &usize) -> usize {
    2 * i + 1
}

fn right(i: &usize) -> usize {
    2 * i + 2
}