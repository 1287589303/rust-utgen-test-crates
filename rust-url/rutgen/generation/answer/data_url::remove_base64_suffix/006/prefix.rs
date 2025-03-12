// Answer 0

#[test]
fn test_remove_base64_suffix_valid_with_en() {
    let input = "Example string that ends with base64;   ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_no_four() {
    let input = "Another string that ends with base6;   ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_no_six() {
    let input = "Another string that ends with base64;   ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_without_base64() {
    let input = "String with missing base64 suffix en string";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_with_tabs() {
    let input = "String with base64; \t\t    ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_with_newlines() {
    let input = "String with newlines\n string with base64; ";
    let result = remove_base64_suffix(input);
}

