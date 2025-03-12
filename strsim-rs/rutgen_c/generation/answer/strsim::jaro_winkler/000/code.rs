// Answer 0

#[test]
fn test_jaro_winkler_identical_strings() {
    assert!((1.0 - jaro_winkler("test", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_different_strings() {
    assert!((0.0 - jaro_winkler("test", "example")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_partial_match() {
    assert!((0.733 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_one_empty_string() {
    assert!((0.0 - jaro_winkler("test", "")).abs() < 0.001);
    assert!((0.0 - jaro_winkler("", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_both_empty_strings() {
    assert!((1.0 - jaro_winkler("", "")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_common_prefix_boost() {
    assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

