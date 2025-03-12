// Answer 0

#[test]
fn test_generic_damerau_levenshtein_a_len_zero() {
    use std::collections::HashMap;

    fn flat_index(row: usize, col: usize, width: usize) -> usize {
        row * width + col
    }

    let a_elems: &[i32] = &[];
    let b_elems: &[i32] = &[2, 3, 1];

    let result = generic_damerau_levenshtein(a_elems, b_elems);
    
    assert_eq!(result, 3);
}

#[test]
fn test_generic_damerau_levenshtein_a_len_zero_with_different_b() {
    use std::collections::HashMap;

    fn flat_index(row: usize, col: usize, width: usize) -> usize {
        row * width + col
    }

    let a_elems: &[i32] = &[];
    let b_elems: &[i32] = &[5, 6];

    let result = generic_damerau_levenshtein(a_elems, b_elems);
    
    assert_eq!(result, 2);
}

