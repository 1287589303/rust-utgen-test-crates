// Answer 0

#[test]
fn test_remove_base64_suffix_no_base64_suffix() {
    let input = "data:text/plain;base64, invalid data";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_incorrect_base64_suffix() {
    let input = "data:text/plain;base64, 46SAB;";
    let result = remove_base64_suffix(input);
}

