// Answer 0

#[test]
fn test_remove_base64_suffix_empty_string() {
    let input = "";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_whitespace_string() {
    let input = "   ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_invalid_start() {
    let input = "Hello World 4";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_invalid_suffix() {
    let input = "data:text/plain;base64,SGVsbG8gd29ybGQhABCD";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_valid_suffix_with_extra_characters() {
    let input = "data:image/png;base64,PNGDATA4E6Esab;extra";
    let result = remove_base64_suffix(input);
}

