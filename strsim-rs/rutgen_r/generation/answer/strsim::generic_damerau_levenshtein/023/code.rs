// Answer 0

#[test]
fn test_empty_a_elements() {
    use std::collections::HashMap;

    let a_elems: Vec<i32> = vec![];
    let b_elems: Vec<i32> = vec![1, 2, 3];
    assert_eq!(b_elems.len(), generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_empty_b_elements() {
    use std::collections::HashMap;

    let a_elems: Vec<i32> = vec![1, 2, 3];
    let b_elems: Vec<i32> = vec![];
    assert_eq!(a_elems.len(), generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_identical_single_element() {
    use std::collections::HashMap;

    let a_elems: Vec<char> = vec!['a'];
    let b_elems: Vec<char> = vec!['a'];
    assert_eq!(0, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_single_insertion() {
    use std::collections::HashMap;

    let a_elems: Vec<char> = vec!['a'];
    let b_elems: Vec<char> = vec!['a', 'b'];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_single_deletion() {
    use std::collections::HashMap;

    let a_elems: Vec<char> = vec!['a', 'b'];
    let b_elems: Vec<char> = vec!['a'];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_single_substitution() {
    use std::collections::HashMap;

    let a_elems: Vec<char> = vec!['a'];
    let b_elems: Vec<char> = vec!['b'];
    assert_eq!(1, generic_damerau_levenshtein(&a_elems, &b_elems));
}

#[test]
fn test_multiple_operations() {
    use std::collections::HashMap;

    let a_elems: Vec<char> = vec!['a', 'b', 'c'];
    let b_elems: Vec<char> = vec!['c', 'b', 'a'];
    assert_eq!(2, generic_damerau_levenshtein(&a_elems, &b_elems));
}

