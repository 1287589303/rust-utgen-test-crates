// Answer 0

fn test_damerau_levenshtein_impl_matching_characters() {
    let s1: Vec<char> = "test".chars().collect();
    let s2: Vec<char> = "test".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 0);
}

fn test_damerau_levenshtein_impl_insert_character() {
    let s1: Vec<char> = "test".chars().collect();
    let s2: Vec<char> = "tester".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 2);
}

fn test_damerau_levenshtein_impl_delete_character() {
    let s1: Vec<char> = "tester".chars().collect();
    let s2: Vec<char> = "test".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 2);
}

fn test_damerau_levenshtein_impl_change_character() {
    let s1: Vec<char> = "test".chars().collect();
    let s2: Vec<char> = "best".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 1);
}

fn test_damerau_levenshtein_impl_transpose_characters() {
    let s1: Vec<char> = "ab".chars().collect();
    let s2: Vec<char> = "ba".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 1);
}

fn test_damerau_levenshtein_impl_empty_strings() {
    let s1: Vec<char> = "".chars().collect();
    let s2: Vec<char> = "".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 0);
}

fn test_damerau_levenshtein_impl_one_empty_string() {
    let s1: Vec<char> = "test".chars().collect();
    let s2: Vec<char> = "".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 4);
}

fn test_damerau_levenshtein_impl_longer_strings() {
    let s1: Vec<char> = "saturday".chars().collect();
    let s2: Vec<char> = "sunday".chars().collect();
    let result = damerau_levenshtein_impl(s1.iter().cloned(), s1.len(), s2.iter().cloned(), s2.len());
    assert_eq!(result, 3);
}

