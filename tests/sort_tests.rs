use algorithms::sort::{bubble::*, heap::*, insertion::*, merge::*, quick::*};
use rand::{thread_rng, Rng};

// Test Arrays and Solutions
fn get_random_array_float() -> [f64; 32] {
    let array = [(); 32].map(|_| thread_rng().gen_range(-100.0..100.0));
    array
}

fn get_random_array_int() -> [i32; 32] {
    let array = [(); 32].map(|_| thread_rng().gen_range(-100..100));
    array
}

fn verify_asc<T: PartialOrd>(array: &[T]) -> bool {
    let mut check = true;
    for i in 1..array.len() {
        if array[i] < array[i - 1] {
            check = false;
            break;
        }
    }

    check
}

fn verify_desc<T: PartialOrd>(array: &[T]) -> bool {
    let mut check = true;
    for i in 1..array.len() {
        if array[i] > array[i - 1] {
            check = false;
            break;
        }
    }

    check
}


// Sort Tests

#[test]
fn test_insertion_sort_asc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    insertion_sort(&mut test_float, true);
    assert!(verify_asc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    insertion_sort(&mut test_int, true);
    assert!(verify_asc(&test_int));
}

#[test]
fn test_insertion_sort_desc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    insertion_sort(&mut test_float, false);
    assert!(verify_desc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    insertion_sort(&mut test_int, false);
    assert!(verify_desc(&test_int));
}

#[test]
fn test_merge_sort_asc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    merge_sort(&mut test_float, 0, 31, true);
    assert!(verify_asc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    merge_sort(&mut test_int, 0, 31, true);
    assert!(verify_asc(&test_int));
}

#[test]
fn test_merge_sort_desc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    merge_sort(&mut test_float, 0, 31, false);
    assert!(verify_desc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    merge_sort(&mut test_int, 0, 31, false);
    assert!(verify_desc(&test_int));
}

#[test]
fn test_bubble_sort_asc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    bubble_sort(&mut test_float, true);
    assert!(verify_asc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    bubble_sort(&mut test_int, true);
    assert!(verify_asc(&test_int));
}

#[test]
fn test_bubble_sort_desc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    bubble_sort(&mut test_float, false);
    assert!(verify_desc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    bubble_sort(&mut test_int, false);
    assert!(verify_desc(&test_int));
}

#[test]
fn test_heap_sort_asc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    heap_sort(&mut test_float, true);
    assert!(verify_asc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    heap_sort(&mut test_int, true);
    assert!(verify_asc(&test_int));
}

#[test]
fn test_heap_sort_desc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    heap_sort(&mut test_float, false);
    assert!(verify_desc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    heap_sort(&mut test_int, false);
    assert!(verify_desc(&test_int));
}

#[test]
fn test_quick_sort_asc() {
    let mut test_float: [f64; 32] = get_random_array_float();
    quick_sort(&mut test_float);
    assert!(verify_asc(&test_float));

    let mut test_int: [i32; 32] = get_random_array_int();
    quick_sort(&mut test_int);
    assert!(verify_asc(&test_int));
}
