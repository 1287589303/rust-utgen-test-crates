// Answer 0

#[test]
fn test_remove_base64_suffix_empty_string() {
    let input = "";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_4() {
    let input = "46base64;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_6() {
    let input = "45base64;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_e() {
    let input = "46base64;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_s() {
    let input = "46base64;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_a() {
    let input = "46base64;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_b() {
    let input = "46base64;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_proper_suffix() {
    let input = "46abc;";
    let result = remove_base64_suffix(input);
}

