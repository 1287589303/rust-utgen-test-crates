// Answer 0

#[test]
fn test_generic_levenshtein_with_non_matching_sequences() {
    let a = &[1, 2, 3];
    let b = &[1, 2, 3, 4, 5, 6];
    assert_eq!(3, generic_levenshtein(&a, &b));
}

#[test]
fn test_generic_levenshtein_with_empty_b() {
    let a = &[1, 2, 3];
    let b: &[i32] = &[];
    assert_eq!(3, generic_levenshtein(&a, &b));
}

#[test]
fn test_generic_levenshtein_with_empty_a() {
    let a: &[i32] = &[];
    let b = &[1, 2, 3];
    assert_eq!(3, generic_levenshtein(&a, &b));
}

#[test]
fn test_generic_levenshtein_with_identical_sequences() {
    let a = &[1, 2, 3];
    let b = &[1, 2, 3];
    assert_eq!(0, generic_levenshtein(&a, &b));
}

#[test]
fn test_generic_levenshtein_with_one_character_difference() {
    let a = &[1, 2, 3];
    let b = &[1, 2, 4];
    assert_eq!(1, generic_levenshtein(&a, &b));
}

