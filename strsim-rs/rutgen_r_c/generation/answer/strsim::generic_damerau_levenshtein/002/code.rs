// Answer 0

#[test]
fn test_generic_damerau_levenshtein_a_len_zero() {
    let a: Vec<i32> = Vec::new();
    let b = vec![1, 2, 3];
    assert_eq!(3, generic_damerau_levenshtein(&a, &b));
}

#[test]
fn test_generic_damerau_levenshtein_a_len_zero_with_empty_b() {
    let a: Vec<i32> = Vec::new();
    let b: Vec<i32> = Vec::new();
    assert_eq!(0, generic_damerau_levenshtein(&a, &b));
}

#[test]
fn test_generic_damerau_levenshtein_a_len_zero_large_b() {
    let a: Vec<i32> = Vec::new();
    let b = vec![1, 2, 3, 4, 5];
    assert_eq!(5, generic_damerau_levenshtein(&a, &b));
}

