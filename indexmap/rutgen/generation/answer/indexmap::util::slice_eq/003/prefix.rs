// Answer 0

#[test]
fn test_slice_eq_equal_length_false_condition() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 4];
    let eq = |a: &i32, b: &i32| *a == *b;

    let result = slice_eq(left, right, eq);
}

#[test]
fn test_slice_eq_equal_length_false_condition_different_types() {
    let left = &["apple", "banana", "cherry"];
    let right = &["apple", "banana", "orange"];
    let eq = |a: &&str, b: &&str| *a == *b;

    let result = slice_eq(left, right, eq);
}

#[test]
fn test_slice_eq_equal_length_false_condition_edge() {
    let left = &[5, 6, 7];
    let right = &[5, 8, 7];
    let eq = |a: &i32, b: &i32| *a == *b;

    let result = slice_eq(left, right, eq);
}

