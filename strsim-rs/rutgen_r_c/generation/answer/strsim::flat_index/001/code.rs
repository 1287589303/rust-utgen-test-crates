// Answer 0

#[test]
fn test_flat_index_zeroes() {
    let result = flat_index(0, 0, 1);
    assert_eq!(result, 0);
}

#[test]
fn test_flat_index_single_element() {
    let result = flat_index(0, 0, 10);
    assert_eq!(result, 0);
}

#[test]
fn test_flat_index_first_row() {
    let result = flat_index(5, 0, 10);
    assert_eq!(result, 5);
}

#[test]
fn test_flat_index_second_row() {
    let result = flat_index(2, 1, 10);
    assert_eq!(result, 12);
}

#[test]
fn test_flat_index_multiple_rows() {
    let result = flat_index(3, 2, 10);
    assert_eq!(result, 23);
}

#[test]
fn test_flat_index_large_values() {
    let result = flat_index(9, 9, 10);
    assert_eq!(result, 99);
}

#[test]
fn test_flat_index_with_height() {
    let result = flat_index(0, 5, 15);
    assert_eq!(result, 75);
}

