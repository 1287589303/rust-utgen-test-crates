// Answer 0

#[test]
fn test_jaro_winkler_identical_strings() {
    assert!((1.0 - jaro_winkler("test", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_completely_different_strings() {
    assert!((0.0 - jaro_winkler("hello", "world")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_partial_match() {
    assert!((0.833 - jaro_winkler("hello", "hallo")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_with_common_prefix() {
    assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_strings_with_different_case() {
    assert!((0.866 - jaro_winkler("Cheese", "cheesy")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_string() {
    assert!((0.0 - jaro_winkler("", "nonempty")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_both_empty_strings() {
    assert!((1.0 - jaro_winkler("", "")).abs() < 0.001);
}

