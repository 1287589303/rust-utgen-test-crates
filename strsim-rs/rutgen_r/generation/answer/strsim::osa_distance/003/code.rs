// Answer 0

#[test]
fn test_osa_distance_case_with_transpositions() {
    let a = "abc";
    let b = "bac";
    assert_eq!(osa_distance(a, b), 2);
}

#[test]
fn test_osa_distance_case_without_transpositions() {
    let a = "abc";
    let b = "abcd";
    assert_eq!(osa_distance(a, b), 1);
}

#[test]
fn test_osa_distance_edge_case_empty_first() {
    let a = "";
    let b = "abc";
    assert_eq!(osa_distance(a, b), 3);
}

#[test]
fn test_osa_distance_edge_case_empty_second() {
    let a = "abc";
    let b = "";
    assert_eq!(osa_distance(a, b), 3);
}

