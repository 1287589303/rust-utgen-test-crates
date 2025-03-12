// Answer 0

#[test]
fn test_osa_distance_basic() {
    let a = "a";
    let b = "b";
    assert_eq!(osa_distance(a, b), 1);
}

#[test]
fn test_osa_distance_single_char_transposition() {
    let a = "ab";
    let b = "ba";
    assert_eq!(osa_distance(a, b), 1);
}

#[test]
fn test_osa_distance_with_empty_second_string() {
    let a = "abc";
    let b = "";
    assert_eq!(osa_distance(a, b), 3);
}

#[test]
fn test_osa_distance_with_empty_first_string() {
    let a = "";
    let b = "abc";
    assert_eq!(osa_distance(a, b), 3);
}

#[test]
fn test_osa_distance_empty_strings() {
    let a = "";
    let b = "";
    assert_eq!(osa_distance(a, b), 0);
}

#[test]
fn test_osa_distance_with_different_lengths() {
    let a = "abc";
    let b = "ab";
    assert_eq!(osa_distance(a, b), 1);
}

#[test]
fn test_osa_distance_high_similarity() {
    let a = "abcdefg";
    let b = "abcfg";
    assert_eq!(osa_distance(a, b), 2);
}

