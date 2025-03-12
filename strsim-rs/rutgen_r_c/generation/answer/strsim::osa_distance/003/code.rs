// Answer 0

#[test]
fn test_osa_distance_boundary_conditions() {
    let a = "abc";
    let b = "def";
    let result = osa_distance(a, b);
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_with_transposition_not_adjacent() {
    let a = "abcd";
    let b = "bacd";
    let result = osa_distance(a, b);
    assert_eq!(result, 2);
}

#[test]
fn test_osa_distance_edge_case_with_transposition() {
    let a = "ab";
    let b = "bca";
    let result = osa_distance(a, b);
    assert_eq!(result, 3);
}

