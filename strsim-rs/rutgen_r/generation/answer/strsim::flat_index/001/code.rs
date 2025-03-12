// Answer 0

#[test]
fn test_flat_index_zero() {
    let i = 0;
    let j = 0;
    let width = 5;
    assert_eq!(flat_index(i, j, width), j * width + i);
}

#[test]
fn test_flat_index_first_element() {
    let i = 1;
    let j = 0;
    let width = 5;
    assert_eq!(flat_index(i, j, width), j * width + i);
}

#[test]
fn test_flat_index_second_row() {
    let i = 0;
    let j = 1;
    let width = 5;
    assert_eq!(flat_index(i, j, width), j * width + i);
}

#[test]
fn test_flat_index_middle() {
    let i = 2;
    let j = 2;
    let width = 5;
    assert_eq!(flat_index(i, j, width), j * width + i);
}

#[test]
fn test_flat_index_last_element() {
    let i = 4;
    let j = 3;
    let width = 5;
    assert_eq!(flat_index(i, j, width), j * width + i);
}

#[test]
fn test_flat_index_large_values() {
    let i = 10;
    let j = 10;
    let width = 100;
    assert_eq!(flat_index(i, j, width), j * width + i);
}

