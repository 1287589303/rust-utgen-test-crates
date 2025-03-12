// Answer 0

#[test]
fn test_damerau_levenshtein_edge_case() {
    let s1: String = "abc".chars();
    let len1: usize = 3;
    let s2: String = "abd".chars();
    let len2: usize = 3;
    
    let result = damerau_levenshtein_impl(s1.clone(), len1, s2.clone(), len2);
}

#[test]
fn test_damerau_levenshtein_case_different_chars() {
    let s1: String = "xyz".chars();
    let len1: usize = 3;
    let s2: String = "xywz".chars();
    let len2: usize = 4;
    
    let result = damerau_levenshtein_impl(s1.clone(), len1, s2.clone(), len2);
}

#[test]
fn test_damerau_levenshtein_max_transpositions() {
    let s1: String = "kitten".chars();
    let len1: usize = 6;
    let s2: String = "sitting".chars();
    let len2: usize = 7;

    let result = damerau_levenshtein_impl(s1.clone(), len1, s2.clone(), len2);
}

#[test]
fn test_damerau_levenshtein_repeated_chars() {
    let s1: String = "hello".chars();
    let len1: usize = 5;
    let s2: String = "hallo".chars();
    let len2: usize = 5;

    let result = damerau_levenshtein_impl(s1.clone(), len1, s2.clone(), len2);
}

