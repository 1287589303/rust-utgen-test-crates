// Answer 0

#[test]
fn test_jaro_winkler_identical_strings() {
    assert!((1.0 - jaro_winkler("test", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_completely_different_strings() {
    assert!((0.0 - jaro_winkler("abc", "xyz")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_partial_match() {
    assert!((0.733 - jaro_winkler("night", "nacht")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_similar_strings() {
    assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_string() {
    assert!((0.0 - jaro_winkler("", "nonempty")).abs() < 0.001);
    assert!((0.0 - jaro_winkler("nonempty", "")).abs() < 0.001);
} 

#[test]
fn test_jaro_winkler_prefix_boost() {
    let score = jaro_winkler("prefix_example", "prefix_test");
    assert!(score > 0.85);
} 

#[test]
fn test_jaro_winkler_case_sensitive() {
    assert!((0.911 - jaro_winkler("Hello", "hello")).abs() < 0.001);
} 

#[test]
fn test_jaro_winkler_varied_length() {
    assert!((0.767 - jaro_winkler("longer_string", "long_string")).abs() < 0.001);
} 

#[test]
fn test_jaro_winkler_single_character() {
    assert!((0.0 - jaro_winkler("a", "b")).abs() < 0.001);
    assert!((1.0 - jaro_winkler("a", "a")).abs() < 0.001);
}

