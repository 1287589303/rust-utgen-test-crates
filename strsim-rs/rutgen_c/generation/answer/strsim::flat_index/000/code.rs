// Answer 0

#[test]
fn test_flat_index_basic() {
    let i = 2;
    let j = 3;
    let width = 5;
    let result = flat_index(i, j, width);
    assert_eq!(result, 17); // (3 * 5) + 2 = 17
}

#[test]
fn test_flat_index_zero_index() {
    let i = 0;
    let j = 0;
    let width = 10;
    let result = flat_index(i, j, width);
    assert_eq!(result, 0); // (0 * 10) + 0 = 0
}

#[test]
fn test_flat_index_last_row() {
    let i = 4;
    let j = 1;
    let width = 5;
    let result = flat_index(i, j, width);
    assert_eq!(result, 9); // (1 * 5) + 4 = 9
}

#[test]
fn test_flat_index_large_values() {
    let i = 100;
    let j = 200;
    let width = 1000;
    let result = flat_index(i, j, width);
    assert_eq!(result, 200100); // (200 * 1000) + 100 = 200100
}

#[test]
fn test_flat_index_with_one() {
    let i = 1;
    let j = 1;
    let width = 1;
    let result = flat_index(i, j, width);
    assert_eq!(result, 1); // (1 * 1) + 1 = 1
}

