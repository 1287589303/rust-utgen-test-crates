// Answer 0

#[test]
fn test_osa_distance_basic() {
    assert_eq!(osa_distance("abc", "abc"), 0);
}

#[test]
fn test_osa_distance_single_transposition() {
    assert_eq!(osa_distance("ab", "ba"), 1); // 'ab' can be turned into 'ba' with one adjacent transposition
}

#[test]
fn test_osa_distance_multiple_transpositions() {
    assert_eq!(osa_distance("abcde", "bacde"), 1); // 'abcde' can become 'bacde' with one adjacent transposition
}

#[test]
fn test_osa_distance_insertion() {
    assert_eq!(osa_distance("abc", "abdc"), 1); // 'abc' can be turned into 'abdc' with one insertion/deletion
}

#[test]
fn test_osa_distance_complex_case() {
    assert_eq!(osa_distance("kitten", "sitting"), 3); // 'kitten' compared to 'sitting' requires 3 operations
}

#[test]
fn test_osa_distance_equal_strings() {
    assert_eq!(osa_distance("rust", "rust"), 0); // Same string distance is zero
}

