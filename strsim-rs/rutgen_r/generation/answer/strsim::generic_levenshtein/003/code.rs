// Answer 0

#[test]
fn test_generic_levenshtein_empty_sequences() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_one_empty_sequence() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_different_length_sequences() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_identical_sequences() {
    let a: Vec<i32> = vec![5, 6, 7];
    let b: Vec<i32> = vec![5, 6, 7];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_substitution_cost() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![3, 2, 1];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

#[test]
fn test_generic_levenshtein_insertion_cost() {
    let a: Vec<i32> = vec![1, 2];
    let b: Vec<i32> = vec![1, 2, 3, 4];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

#[test]
fn test_generic_levenshtein_deletion_cost() {
    let a: Vec<i32> = vec![1, 2, 3, 4];
    let b: Vec<i32> = vec![1, 2];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

