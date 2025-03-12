// Answer 0

#[test]
fn test_generic_levenshtein_basic_case() {
    let a = &[1, 2, 3];
    let b = &[1, 2, 3, 4, 5, 6];
    assert_eq!(3, generic_levenshtein(a, b));
}

#[test]
fn test_generic_levenshtein_empty_strings() {
    let a: &[i32] = &[];
    let b: &[i32] = &[];
    assert_eq!(0, generic_levenshtein(a, b));
}

#[test]
fn test_generic_levenshtein_one_empty_string() {
    let a = &[1, 2, 3];
    let b: &[i32] = &[];
    assert_eq!(3, generic_levenshtein(a, b));

    let a: &[i32] = &[];
    let b = &[1, 2, 3];
    assert_eq!(3, generic_levenshtein(a, b));
}

#[test]
fn test_generic_levenshtein_identical_strings() {
    let a = &[1, 2, 3];
    let b = &[1, 2, 3];
    assert_eq!(0, generic_levenshtein(a, b));
}

#[test]
fn test_generic_levenshtein_substitution_only() {
    let a = &[1, 2, 3];
    let b = &[3, 2, 1];
    assert_eq!(3, generic_levenshtein(a, b));
}

#[test]
fn test_generic_levenshtein_insertion_and_deletion() {
    let a = &[1, 2];
    let b = &[1, 2, 3, 4];
    assert_eq!(2, generic_levenshtein(a, b));
}

#[test]
fn test_generic_levenshtein_complex_case() {
    let a = &[1, 2, 3, 4, 5];
    let b = &[1, 3, 2, 5, 6];
    assert_eq!(3, generic_levenshtein(a, b));
}

