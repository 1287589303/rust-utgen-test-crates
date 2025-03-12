// Answer 0

#[test]
fn test_osa_distance_with_adjacent_transpositions() {
    let a = "abc";
    let b = "acb";
    let result = osa_distance(a, b);
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_with_multiple_transpositions() {
    let a = "abcd";
    let b = "abdc";
    let result = osa_distance(a, b);
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_with_transpositions_and_insertions() {
    let a = "abcde";
    let b = "abdcf";
    let result = osa_distance(a, b);
    assert_eq!(result, 2);
}

#[test]
fn test_osa_distance_with_no_changes() {
    let a = "abc";
    let b = "abc";
    let result = osa_distance(a, b);
    assert_eq!(result, 0);
}

#[test]
fn test_osa_distance_with_empty_string() {
    let a = "abc";
    let b = "";
    let result = osa_distance(a, b);
    assert_eq!(result, 3);
}

