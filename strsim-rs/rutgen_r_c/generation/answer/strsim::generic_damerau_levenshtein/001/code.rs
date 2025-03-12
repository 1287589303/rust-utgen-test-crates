// Answer 0

#[test]
fn test_generic_damerau_levenshtein_empty_first() {
    let a: &[i32] = &[];
    let b: &[i32] = &[1, 2, 3];
    let result = generic_damerau_levenshtein(a, b);
    assert_eq!(result, b.len());
}

#[test]
fn test_generic_damerau_levenshtein_empty_second() {
    let a: &[i32] = &[1, 2, 3];
    let b: &[i32] = &[];
    let result = generic_damerau_levenshtein(a, b);
    assert_eq!(result, a.len());
}

#[test]
fn test_generic_damerau_levenshtein_no_operations_needed() {
    let a: &[i32] = &[1, 2, 3];
    let b: &[i32] = &[1, 2, 3];
    let result = generic_damerau_levenshtein(a, b);
    assert_eq!(result, 0);
}

#[test]
fn test_generic_damerau_levenshtein_insertion_needed() {
    let a: &[i32] = &[1, 2];
    let b: &[i32] = &[2, 3, 1];
    let result = generic_damerau_levenshtein(a, b);
    assert_eq!(result, 2);
}

#[test]
fn test_generic_damerau_levenshtein_deletion_needed() {
    let a: &[i32] = &[1, 2, 3];
    let b: &[i32] = &[2, 3];
    let result = generic_damerau_levenshtein(a, b);
    assert_eq!(result, 1);
}

#[test]
fn test_generic_damerau_levenshtein_transposition_needed() {
    let a: &[i32] = &[1, 2, 3];
    let b: &[i32] = &[2, 1, 3];
    let result = generic_damerau_levenshtein(a, b);
    assert_eq!(result, 1);
}

