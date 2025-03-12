// Answer 0

#[test]
fn test_generic_damerau_levenshtein_empty_strings() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_damerau_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_damerau_levenshtein_first_empty() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![1, 2, 3];
    assert_eq!(generic_damerau_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_damerau_levenshtein_second_empty() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![];
    assert_eq!(generic_damerau_levenshtein(&a, &b), 3);
}

#[test]
fn test_generic_damerau_levenshtein_no_matches() {
    let a: Vec<i32> = vec![1];
    let b: Vec<i32> = vec![2];
    assert_eq!(generic_damerau_levenshtein(&a, &b), 2);
}

#[test]
fn test_generic_damerau_levenshtein_one_char_match() {
    let a: Vec<i32> = vec![1];
    let b: Vec<i32> = vec![1];
    assert_eq!(generic_damerau_levenshtein(&a, &b), 0);
}

#[test]
fn test_generic_damerau_levenshtein_multiple_elements() {
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![3, 2, 1];
    assert_eq!(generic_damerau_levenshtein(&a, &b), 4);
}

