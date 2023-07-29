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
/// # use crate::algorithms::sort::quick::quick_sort;
/// // Ascending Sort
/// let mut array = [-1, 5, 4, 1, 0];
/// quick_sort(&mut array);
/// 
/// assert_eq!(array, [-1, 0, 1, 4, 5]);
/// ```

pub fn quick_sort<T>(data: &mut [T])
where   
    T: PartialOrd + Copy 
{
    let length = data.len() as isize;
    _quick_sort(data, 0, length - 1);
}


fn _quick_sort<T>(data: &mut [T], left: isize, right: isize) 
where   
    T: PartialOrd + Copy
{
    if left < right {
        let pivot_index = partition(data, left, right);
        _quick_sort(data, left, pivot_index - 1);
        _quick_sort(data, pivot_index + 1, right);
    }
}

pub fn partition<T>(data: &mut [T], left: isize, right: isize) -> isize 
where   
    T: PartialOrd + Copy
{
    let x = data[right as usize];
    let mut i = left;

    for j in left..right {
        if data[j as usize] <= x {
            data.swap(i as usize, j as usize);
            i += 1;
        }
    }

    data.swap(i as usize, right as usize);
    i
}
