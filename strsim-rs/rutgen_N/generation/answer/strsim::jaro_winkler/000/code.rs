// Answer 0

#[test]
fn test_jaro_winkler_common_prefix() {
    assert!((0.866 - jaro_winkler("cheeseburger", "cheese fries")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_no_common_prefix() {
    assert!((0.0 - jaro_winkler("apple", "orange")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_identical_strings() {
    assert!((1.0 - jaro_winkler("test", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_empty_strings() {
    assert!((0.0 - jaro_winkler("", "")).abs() < 0.001);
}

#[test]
fn test_jaro_winkler_one_empty_string() {
    assert!((0.0 - jaro_winkler("hello", "")).abs() < 0.001);
    assert!((0.0 - jaro_winkler("", "world")).abs() < 0.001);
}

