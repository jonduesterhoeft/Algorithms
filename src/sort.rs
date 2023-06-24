pub fn insertion_sort(data: &mut Vec<i32>, asc: bool) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_asc() {
        let mut test_data = vec![-1, 5, 4, 1, 0];
        insertion_sort(&mut test_data, true);
        let expected = vec![-1, 0, 1, 4, 5];
        assert_eq!(test_data, expected);
    }

    #[test]
    fn test_insertion_sort_desc() {
        let mut test_data = vec![-1, 5, 4, 1, 0];
        insertion_sort(&mut test_data, false);
        let expected = vec![5, 4, 1, 0, -1];
        assert_eq!(test_data, expected);
    }
}
