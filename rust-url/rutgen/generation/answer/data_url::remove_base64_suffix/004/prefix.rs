// Answer 0

#[test]
fn test_remove_base64_suffix_invalid_suffix_1() {
    let input = "SGVsbG8gd29ybGQ=        ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_invalid_suffix_2() {
    let input = "SGVsbG8gd29ybGQ=;\t\n";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_invalid_suffix_with_invalid_characters() {
    let input = "SGVsbG8gd29ybGQ=;%#@! ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_invalid_suffix_no_suffix() {
    let input = "SGVsbG8gd29ybGQ= ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_with_only_whitespace() {
    let input = "SGVsbG8gd29ybGQ=     ";
    let result = remove_base64_suffix(input);
}

