// Answer 0

#[test]
fn test_osa_distance_non_empty_a_empty_b() {
    let a = "a"; 
    let b = ""; 
    let distance = osa_distance(a, b);
}

#[test]
fn test_osa_distance_multiple_chars_a_empty_b() {
    let a = "abc"; 
    let b = ""; 
    let distance = osa_distance(a, b);
}

#[test]
fn test_osa_distance_single_char_a_empty_b() {
    let a = "z"; 
    let b = ""; 
    let distance = osa_distance(a, b);
}

