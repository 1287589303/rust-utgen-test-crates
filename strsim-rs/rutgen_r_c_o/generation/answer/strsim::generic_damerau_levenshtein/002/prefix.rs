// Answer 0

#[test]
fn test_generic_damerau_levenshtein_a_len_zero_b_len_one() {
    let a_elems: Vec<i32> = vec![];
    let b_elems: Vec<i32> = vec![1];
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_a_len_zero_b_len_ten() {
    let a_elems: Vec<i32> = vec![];
    let b_elems: Vec<i32> = (1..=10).collect();
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

#[test]
fn test_generic_damerau_levenshtein_a_len_zero_b_len_hundred() {
    let a_elems: Vec<i32> = vec![];
    let b_elems: Vec<i32> = (1..=100).collect();
    generic_damerau_levenshtein(&a_elems, &b_elems);
}

