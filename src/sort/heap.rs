/// Uses the **heap sort** algorithm to sort an array.
/// 
/// Heap sort combines the better qualities of insertion and merge sorts, 
/// with a fast running time and sort-in-place.
/// 
/// Worst-Case Running Time: O(*n* lg *n*)
/// 
/// Note that this function sorts the array directly *in place*.
/// 
/// # Examples
/// 
/// ```
/// # use crate::algorithms::sort::heap::heap_sort;
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// heap_sort(&mut array, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// ```
pub fn heap_sort<T>(data: &mut [T], asc: bool) 
where 
    T: PartialOrd
{
    build_heap(data, asc);

    let mut heap_size = data.len(); 

    for i in (1..heap_size).rev() {
        data.swap(0, i);
        heap_size -= 1;

        if asc {
            max_heapify(data, &0, &heap_size);
        } else {
            min_heapify(data, &0, &heap_size);
        }
    }
}

// Converts an array into either a max- or min- heap
fn build_heap<T>(data: &mut [T], asc: bool)
where 
    T: PartialOrd
{
    let heap_size = data.len();
    for i in (0..=(heap_size / 2)).rev() {
        if asc {
            max_heapify(data, &i, &heap_size);
        } else {
            min_heapify(data, &i, &heap_size);
        }
    }
}

// Maintains the max-heap property
fn max_heapify<T>(data: &mut [T], i: &usize, heap_size: &usize) 
where 
    T: PartialOrd
{
    let l = left(i);
    let r = right(i);
    let mut largest: usize;

    if l < *heap_size && data[l] > data[*i] {
        largest = l;
    } else {
        largest = *i;
    }

    if r < *heap_size && data[r] > data[largest] {
        largest = r;
    }

    if largest != *i {
        data.swap(*i, largest);
        max_heapify(data, &largest, heap_size);
    }
}

// Maintains the min-heap property
fn min_heapify<T>(data: &mut [T], i: &usize, heap_size: &usize) 
where 
    T: PartialOrd
{
    let l = left(i);
    let r = right(i);
    let mut smallest: usize;

    if l < *heap_size && data[l] < data[*i] {
        smallest = l;
    } else {
        smallest = *i;
    }

    if r < *heap_size && data[r] < data[smallest] {
        smallest = r;
    }

    if smallest != *i {
        data.swap(*i, smallest);
        min_heapify(data, &smallest, heap_size);
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