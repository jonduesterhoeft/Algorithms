/// Uses the **insertion sort** algorithm to sort an array.
/// 
/// Insertion sort is an efficient algorithm for a small number of elements.
/// 
/// Worst-Case Running Time: Θ(*n*<sup>2</sup>)
/// 
/// 
/// Average-Case Running Time: Θ(*n*<sup>2</sup>)
/// 
/// Note that this function sorts the array directly *in place*.
/// 
/// # Examples
/// 
/// ```
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::insertion_sort(&mut array, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::insertion_sort(&mut array, false);
///
/// assert_eq!(array, [5, 4, 1, 0, -1]);
/// ```
///
pub fn insertion_sort<T>(data: &mut [T], asc: bool)
where 
    T: PartialOrd
{
    for i in 1..data.len() {
        let mut j: usize = i;
        if asc {
            while j > 0 && data[j - 1] > data[j] {
                data.swap(j - 1, j);
                j = j - 1;
            }
        } else {
            while j > 0 && data[j - 1] < data[j] {
                data.swap(j - 1, j);
                j = j - 1;
            }
        }
    }
}


/// Uses the **merge sort** algorithm to sort an array.
/// 
/// Merge sort uses a recurrsive divide-and-conquer method to sort an array.
/// 
/// Worst-Case Running Time: Θ(*n* lg *n*)
/// 
/// 
/// Average-Case Running Time: Θ(*n* lg *n*)
/// 
/// Note that this function sorts the array directly *in place*.
/// 
/// # Examples
/// 
/// ```
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::merge_sort(&mut array, 0, 4, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::merge_sort(&mut array, 0, 4, false);
///
/// assert_eq!(array, [5, 4, 1, 0, -1]);
/// ```
pub fn merge_sort<T>(data: &mut [T], p: usize, r: usize, asc: bool) 
where 
    T: PartialOrd + Copy
{
    if p >= r {
        return;
    }
    let q = (p + r) / 2;
    merge_sort(data, p, q, asc);
    merge_sort(data, q + 1, r, asc);
    merge(data, p, q, r, asc);
}

fn merge<T>(data: &mut [T], p: usize, q: usize, r: usize, asc: bool) 
where 
    T: PartialOrd + Copy
{
    // Split data into two arrays and create copies
    let left = data[p..=q].to_owned();
    let right = data[q + 1..=r].to_owned();

    // counter variables
    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < left.len() && j < right.len() {
        if asc && left[i] <= right[j] {
            data[k] = left[i];
            i = i + 1;
        } else if !asc && left[i] >= right[j] {
            data[k] = left[i];
            i = i + 1;
        } else {
            data[k] = right[j];
            j = j + 1;
        }
        k = k + 1;
    }

    while i < left.len() {
        data[k] = left[i];
        i = i + 1;
        k = k + 1;
    }

    while j < right.len() {
        data[k] = right[j];
        j = j + 1;
        k = k + 1;
    }
}


/// Uses the **bubble sort** algorithm to sort an array.
/// 
/// Bubble sort is a popular but inefficient sorting algorithm.
/// 
/// Worst-Case Running Time: Θ(*n*<sup>2</sup>)
/// 
/// 
/// Average-Case Running Time: Θ(*n*<sup>2</sup>)
/// 
/// Note that this function sorts the array directly *in place*.
/// 
/// # Examples
/// 
/// ```
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::bubble_sort(&mut array, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::bubble_sort(&mut array, false);
///
/// assert_eq!(array, [5, 4, 1, 0, -1]);
/// ```
pub fn bubble_sort<T>(data: &mut [T], asc: bool) 
where
    T: PartialOrd
{
    for i in 0..data.len() {
        for j in ((i + 1)..data.len()).rev() {
            if asc && data[j] < data[j - 1] {
                data.swap(j, j - 1);
            } else if !asc && data[j] > data[j - 1] {
                data.swap(j, j - 1);
            }
        }
    }
}


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
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::heap_sort(&mut array);
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

fn parent(i: &usize) -> usize {
    (i - 1) / 2
}

fn left(i: &usize) -> usize {
    2 * i + 1
}

fn right(i: &usize) -> usize {
    2 * i + 2
}


/// Uses the **quick sort** algorithm to sort an array.
/// 
/// Despite the worst-case running time, quick sort is often the best practical
/// choice for sorting because it is very efficient on average.
/// 
/// Worst-Case Running Time: Θ(*n*<sup>2</sup>)
/// 
/// 
/// Average-Case Running Time: Θ(*n* lg *n*) (expected)
/// 
/// Note that this function sorts the array directly *in place*.
/// 
/// # Examples
/// 
/// ```
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// algorithms::sort::quick_sort(&mut array, 0 ,4);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// ```
pub fn quick_sort<T>(data: &mut [T], p: usize, r: usize) 
where   
    T: PartialOrd + Copy
{
    if p < r {
        let q = partition(data, &p, &r);
        quick_sort(data, p, q - 1);
        quick_sort(data, q + 1, r);
    }
}

fn partition<T>(data: &mut [T], p: &usize, r: &usize) -> usize 
where   
    T: PartialOrd + Copy
{
    let x = data[*r];
    let mut i: isize = *p as isize - 1;

    for j in *p..=(*r - 1) {
        if data[j] <= x {
            i += 1;
            data.swap(i as usize, j);
        }
    }

    data.swap(i as usize + 1, *r);
    i as usize + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_asc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        insertion_sort(&mut test_data, true);
        let expected = [-1, 0, 1, 4, 5];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_insertion_sort_desc() {
        let mut test_data = [-1.0, 5.0, 4.0, 1.0, 0.0];
        insertion_sort(&mut test_data, false);
        let expected = [5.0, 4.0, 1.0, 0.0, -1.0];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_merge_sort_asc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        merge_sort(&mut test_data, 0, 4, true);
        let expected = [-1, 0, 1, 4, 5];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_merge_sort_desc() {
        let mut test_data = vec![-1, 5, 4, 1, 0];
        merge_sort(&mut test_data, 0, 4, false);
        let expected = vec![5, 4, 1, 0, -1];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_bubble_sort_asc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        bubble_sort(&mut test_data, true);
        let expected = [-1, 0, 1, 4, 5];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_bubble_sort_desc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        bubble_sort(&mut test_data, false);
        let expected = [5, 4, 1, 0, -1];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_heap_sort_asc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        heap_sort(&mut test_data, true);
        let expected = [-1, 0, 1, 4, 5];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_heap_sort_desc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        heap_sort(&mut test_data, false);
        let expected = [5, 4, 1, 0, -1];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_quick_sort_asc() {
        let mut test_data = [-1, 5, 4, 1, 0];
        quick_sort(&mut test_data, 0, 4);
        let expected = [-1, 0, 1, 4, 5];
        assert_eq!(test_data, expected);
    }
}
