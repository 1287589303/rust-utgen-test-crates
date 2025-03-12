// Answer 0

#[test]
fn test_generic_damerau_levenshtein_with_empty_first_vector() {
    let a_elems: Vec<i32> = vec![];
    let b_elems: Vec<i32> = vec![1, 2, 3];
    assert_eq!(3, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_with_empty_second_vector() {
    let a_elems: Vec<i32> = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![];
    assert_eq!(3, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_same_elements() {
    let a_elems: Vec<i32> = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![1, 2, 3];
    assert_eq!(0, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_different_elements() {
    let a_elems: Vec<i32> = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![4, 5, 6];
    assert_eq!(6, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_substitution() {
    let a_elems: Vec<i32> = vec![1, 2];
    let b_elems: Vec<i32> = vec![1, 3];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_insertion() {
    let a_elems: Vec<i32> = vec![1, 2];
    let b_elems: Vec<i32> = vec![1, 2, 3];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_deletion() {
    let a_elems: Vec<i32> = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![1, 2];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_generic_damerau_levenshtein_transposition() {
    let a_elems: Vec<i32> = vec![1, 2, 3, 4];
    let b_elems: Vec<i32> = vec![1, 3, 2, 4];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

