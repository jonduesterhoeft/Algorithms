/// Uses the **insertion sort** algorithm to sort an array.
/// 
/// Insertion sort is an efficient algorithm for a small number
/// of elements. 
/// 
/// # Examples
/// 
/// ```
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// insertion_sort(&mut array, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// insertion_sort(&mut test_data, false);
///
/// assert_eq!(array, [5, 4, 1, 0, -1]);
/// ```
pub fn insertion_sort(data: &mut [i32], asc: bool) {
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


pub fn merge(data: &mut [i32], p: usize, q: usize, r: usize, asc: bool) {
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

pub fn merge_sort(data: &mut [i32], p: usize, r: usize, asc: bool) {
    if p >= r {
        return;
    }
    let q = (p + r) / 2;
    merge_sort(data, p, q, asc);
    merge_sort(data, q + 1, r, asc);
    merge(data, p, q, r, asc);
}


pub fn bubble_sort(data: &mut [i32], asc: bool) {
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


pub fn parent(i: &usize) -> usize {
    (i - 1) / 2
}

pub fn left(i: &usize) -> usize {
    2 * i + 1
}

pub fn right(i: &usize) -> usize {
    2 * i + 2
}

pub fn max_heapify(data: &mut [i32], i: &usize, heap_size: &usize) {
    // Maintains the max-heap property on a array
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

pub fn build_max_heap(data: &mut [i32]) {
    // Converts an array into a max heap using max_heapify
    let heap_size = data.len();
    for i in (0..=(heap_size / 2)).rev() {
        max_heapify(data, &i, &heap_size);
    }
}

pub fn heap_sort(data: &mut [i32]) {
    // The heap sort algorithm
    build_max_heap(data);

    let mut heap_size = data.len(); // heap size

    for i in (1..heap_size).rev() {
        data.swap(0, i);
        heap_size -= 1;
        max_heapify(data, &0, &heap_size);
    }
}

// TODO - Fix (i)
pub fn partition(data: &mut [i32], p: &usize, r: &usize) -> usize {
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

pub fn quick_sort(data: &mut [i32], p: usize, r: usize) {
    if p < r {
        let q = partition(data, &p, &r);
        quick_sort(data, p, q - 1);
        quick_sort(data, q + 1, r);
    }
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
        let mut test_data = [-1, 5, 4, 1, 0];
        insertion_sort(&mut test_data, false);
        let expected = [5, 4, 1, 0, -1];
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
        heap_sort(&mut test_data);
        let expected = [-1, 0, 1, 4, 5];
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
