// Answer 0

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b() {
    let a = "";
    let b = "second";
    let expected = 1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64);
    
    assert!((normalized_levenshtein(a, b) - expected).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_different() {
    let a = "";
    let b = "different";
    let expected = 1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64);
    
    assert!((normalized_levenshtein(a, b) - expected).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_numeric() {
    let a = "";
    let b = "12345";
    let expected = 1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64);
    
    assert!((normalized_levenshtein(a, b) - expected).abs() < 0.00001);
}

#[test]
fn test_normalized_levenshtein_empty_a_non_empty_b_special_chars() {
    let a = "";
    let b = "@#$%^&*";
    let expected = 1.0 - (levenshtein(a, b) as f64) / (a.chars().count().max(b.chars().count()) as f64);

    assert!((normalized_levenshtein(a, b) - expected).abs() < 0.00001);
}

