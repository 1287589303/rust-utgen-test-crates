// Answer 0

#[test]
fn test_osa_distance_with_adjacent_transpositions() {
    let a = "abc";
    let b = "bca";
    let result = osa_distance(a, b);
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_with_multiple_transpositions() {
    let a = "abcdef";
    let b = "bacdfe";
    let result = osa_distance(a, b);
    assert_eq!(result, 5);
}

#[test]
fn test_osa_distance_with_single_character_diff() {
    let a = "x";
    let b = "y";
    let result = osa_distance(a, b);
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_with_no_transpositions() {
    let a = "hello";
    let b = "world";
    let result = osa_distance(a, b);
    assert_eq!(result, 5);
}

#[test]
fn test_osa_distance_with_empty_string() {
    let a = "";
    let b = "abc";
    let result = osa_distance(a, b);
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_with_identical_strings() {
    let a = "test";
    let b = "test";
    let result = osa_distance(a, b);
    assert_eq!(result, 0);
}

