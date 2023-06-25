pub fn insertion_sort(data: &mut [i32], asc: bool) {
    for i in 1..data.len() {
        let mut j = i;
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
        } else if !asc && left[i] >= right[j]{
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
}
