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
/// # use crate::algorithms::sort::bubble::bubble_sort;
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// bubble_sort(&mut array, true);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// 
/// // Descending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// bubble_sort(&mut array, false);
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