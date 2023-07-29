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
/// # use crate::algorithms::sort::merge::merge_sort;
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// merge_sort(&mut array, 0, 4, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// merge_sort(&mut array, 0, 4, false);
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