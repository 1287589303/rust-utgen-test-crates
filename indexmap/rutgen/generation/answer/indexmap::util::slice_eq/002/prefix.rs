// Answer 0

#[test]
fn test_slice_eq_matching_elements() {
    let left = [1, 2, 3];
    let right = [1, 2, 3];
    let eq = |a: &i32, b: &i32| a == b;
    let result = slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_matching_elements_with_zero() {
    let left = [0, 1, 2];
    let right = [0, 1, 2];
    let eq = |a: &i32, b: &i32| a == b;
    let result = slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_matching_strings() {
    let left = ["hello", "world"];
    let right = ["hello", "world"];
    let eq = |a: &str, b: &str| a == b;
    let result = slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_matching_floats() {
    let left = [1.0, 2.0, 3.0];
    let right = [1.0, 2.0, 3.0];
    let eq = |a: &f64, b: &f64| a == b;
    let result = slice_eq(&left, &right, eq);
}

#[test]
fn test_slice_eq_identical_references() {
    let a = 42;
    let left = [&a, &a];
    let right = [&a, &a];
    let eq = |a: &&i32, b: &&i32| a == b;
    let result = slice_eq(&left, &right, eq);
}

