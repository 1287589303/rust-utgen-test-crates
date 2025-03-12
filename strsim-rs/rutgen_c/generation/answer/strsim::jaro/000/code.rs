// Answer 0

#[test]
fn test_jaro_identical_strings() {
    let result = jaro("example", "example");
    assert!((1.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_completely_different_strings() {
    let result = jaro("abc", "def");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_partial_match() {
    let result = jaro("hello", "hallo");
    assert!((0.866 - result).abs() < 0.001);
}

#[test]
fn test_jaro_with_empty_string() {
    let result = jaro("", "nonempty");
    assert!((0.0 - result).abs() < 0.001);
    let result = jaro("nonempty", "");
    assert!((0.0 - result).abs() < 0.001);
}

#[test]
fn test_jaro_with_single_char() {
    let result = jaro("a", "a");
    assert!((1.0 - result).abs() < 0.001);
    let result = jaro("a", "b");
    assert!((0.0 - result).abs() < 0.001);
}

