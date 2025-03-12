// Answer 0

#[test]
fn test_osa_distance_empty_b() {
    let a = "abc";
    let b = "";
    assert_eq!(osa_distance(a, b), 3);
}

#[test]
fn test_osa_distance_empty_a() {
    let a = "";
    let b = "xyz";
    assert_eq!(osa_distance(a, b), 3);
}

#[test]
fn test_osa_distance_no_chars_different() {
    let a = "a";
    let b = "b";
    assert_eq!(osa_distance(a, b), 1);
}

#[test]
fn test_osa_distance_single_char_same() {
    let a = "a";
    let b = "a";
    assert_eq!(osa_distance(a, b), 0);
}

#[test]
fn test_osa_distance_two_chars_transposed() {
    let a = "ab";
    let b = "ba";
    assert_eq!(osa_distance(a, b), 1);
}

#[test]
fn test_osa_distance_longer_strings() {
    let a = "kitten";
    let b = "sitting";
    assert_eq!(osa_distance(a, b), 5);
}

