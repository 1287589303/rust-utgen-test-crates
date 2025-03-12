// Answer 0

#[test]
fn test_slice_eq_different_length_1() {
    let left = &[1, 2, 3];
    let right = &[1, 2];
    let result = slice_eq(left, right, |a, b| a == b);
}

#[test]
fn test_slice_eq_different_length_2() {
    let left = &["a", "b"];
    let right = &["a", "b", "c"];
    let result = slice_eq(left, right, |a, b| a == b);
}

#[test]
fn test_slice_eq_different_length_3() {
    let left = &[3.14, 2.71];
    let right = &[3.14];
    let result = slice_eq(left, right, |a, b| (a - b).abs() < f64::EPSILON);
}

#[test]
fn test_slice_eq_different_length_4() {
    let left = &[true, false];
    let right = &[false];
    let result = slice_eq(left, right, |a, b| a == b);
}

#[test]
fn test_slice_eq_different_length_5() {
    let left: &[i32] = &[];
    let right: &[i32] = &[1];
    let result = slice_eq(left, right, |a, b| a == b);
}

