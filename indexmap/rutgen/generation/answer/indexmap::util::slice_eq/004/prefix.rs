// Answer 0

#[test]
fn test_slice_eq_equal_elements() {
    let left = &[1, 2, 3];
    let right = &[1, 2, 3];
    let result = slice_eq(left, right, |a, b| a == b);
}

#[test]
fn test_slice_eq_equal_strings() {
    let left = &["hello", "world"];
    let right = &["hello", "world"];
    let result = slice_eq(left, right, |a, b| a == b);
}

#[test]
fn test_slice_eq_equal_floats() {
    let left = &[1.1, 2.2, 3.3];
    let right = &[1.1, 2.2, 3.3];
    let result = slice_eq(left, right, |a, b| (a - b).abs() < f64::EPSILON);
}

