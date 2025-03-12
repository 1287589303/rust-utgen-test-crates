// Answer 0

#[test]
fn test_osa_distance_basic() {
    let a = "abc";
    let b = "bca";
    assert_eq!(3, osa_distance(a, b));
}

#[test]
fn test_osa_distance_with_transpositions() {
    let a = "ab";
    let b = "bca";
    assert_eq!(3, osa_distance(a, b));
}

#[test]
fn test_osa_distance_with_empty_string() {
    let a = "a";
    let b = "";
    assert_eq!(1, osa_distance(a, b));
}

#[test]
fn test_osa_distance_identical_strings() {
    let a = "test";
    let b = "test";
    assert_eq!(0, osa_distance(a, b));
}

#[test]
fn test_osa_distance_different_strings() {
    let a = "hello";
    let b = "world";
    assert_eq!(5, osa_distance(a, b));
}

#[test]
fn test_osa_distance_boundary_case() {
    let a = "xyz";
    let b = "xy";
    assert_eq!(1, osa_distance(a, b));
}

