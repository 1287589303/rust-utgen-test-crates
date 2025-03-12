// Answer 0

#[test]
fn test_jaro_identical_strings() {
    assert!((1.0 - jaro("test", "test")).abs() < 0.001);
}

#[test]
fn test_jaro_completely_different_strings() {
    assert!((0.0 - jaro("abc", "def")).abs() < 0.001);
}

#[test]
fn test_jaro_partial_match() {
    assert!((0.7 - jaro("dixon", "dicksonx")).abs() < 0.001);
}

#[test]
fn test_jaro_with_space() {
    assert!((0.866 - jaro("austin powers", "austin powers ")).abs() < 0.001);
}

#[test]
fn test_jaro_empty_strings() {
    assert!((0.0 - jaro("", "")).abs() < 0.001);
}

#[test]
fn test_jaro_boundary_conditions() {
    assert!((0.0 - jaro("a", "")).abs() < 0.001);
    assert!((0.0 - jaro("", "b")).abs() < 0.001);
}

#[test]
fn test_jaro_with_numbers() {
    assert!((0.857 - jaro("12345", "12345")).abs() < 0.001);
    assert!((0.5 - jaro("12345", "54321")).abs() < 0.001);
}

