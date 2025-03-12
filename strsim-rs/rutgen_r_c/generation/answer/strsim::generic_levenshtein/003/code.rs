// Answer 0

#[test]
fn test_generic_levenshtein_empty_strings() {
    let a: Vec<u32> = vec![];
    let b: Vec<u32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_a_empty() {
    let a: Vec<u32> = vec![];
    let b: Vec<u32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_b_empty() {
    let a: Vec<u32> = vec![1, 2, 3];
    let b: Vec<u32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_identical() {
    let a: Vec<u32> = vec![1, 2, 3];
    let b: Vec<u32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_single_difference() {
    let a: Vec<u32> = vec![1, 2, 3];
    let b: Vec<u32> = vec![1, 2, 4];
    assert_eq!(generic_levenshtein(&a, &b), 1);
}

#[test]
fn test_generic_levenshtein_multiple_differences() {
    let a: Vec<u32> = vec![1, 2, 3];
    let b: Vec<u32> = vec![4, 5, 6];
    assert_eq!(generic_levenshtein(&a, &b), 6);
}

#[test]
fn test_generic_levenshtein_insertion() {
    let a: Vec<u32> = vec![1, 2];
    let b: Vec<u32> = vec![1, 2, 3, 4];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

#[test]
fn test_generic_levenshtein_deletion() {
    let a: Vec<u32> = vec![1, 2, 3, 4];
    let b: Vec<u32> = vec![1, 2];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

