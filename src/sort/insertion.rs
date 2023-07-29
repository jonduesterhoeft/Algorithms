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
/// # use crate::algorithms::sort::insertion::insertion_sort;
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// insertion_sort(&mut array, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// insertion_sort(&mut array, false);
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