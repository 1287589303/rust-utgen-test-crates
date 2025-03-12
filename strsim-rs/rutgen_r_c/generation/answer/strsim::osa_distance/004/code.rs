// Answer 0

#[test]
fn test_osa_distance_adjacent_transpositions() {
    let a = "ab";
    let b = "bca";
    assert_eq!(3, osa_distance(a, b));
}

#[test]
fn test_osa_distance_no_transpositions() {
    let a = "abc";
    let b = "def";
    assert_eq!(6, osa_distance(a, b));
}

#[test]
fn test_osa_distance_empty_first() {
    let a = "";
    let b = "abc";
    assert_eq!(3, osa_distance(a, b));
}

#[test]
fn test_osa_distance_empty_second() {
    let a = "abc";
    let b = "";
    assert_eq!(3, osa_distance(a, b));
}

#[test]
fn test_osa_distance_identical_strings() {
    let a = "test";
    let b = "test";
    assert_eq!(0, osa_distance(a, b));
}

#[test]
fn test_osa_distance_single_character_difference() {
    let a = "a";
    let b = "b";
    assert_eq!(1, osa_distance(a, b));
}

#[test]
fn test_osa_distance_transposition_case() {
    let a = "abc";
    let b = "acb";
    assert_eq!(1, osa_distance(a, b));
}

