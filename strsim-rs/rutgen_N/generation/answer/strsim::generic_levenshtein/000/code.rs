// Answer 0

#[test]
fn test_generic_levenshtein_empty_seq() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_one_empty_seq() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_levenshtein(&a, &b), 3);
    
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_identical_seqs() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_levenshtein_substitutions() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![3, 2, 1];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_insertions_and_deletions() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(generic_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_levenshtein_only_insertions() {
    let a: Vec<i32> = vec![1, 3];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_levenshtein(&a, &b), 1);
}

#[test]
fn test_generic_levenshtein_only_deletions() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![1];
    assert_eq!(generic_levenshtein(&a, &b), 2);
}

