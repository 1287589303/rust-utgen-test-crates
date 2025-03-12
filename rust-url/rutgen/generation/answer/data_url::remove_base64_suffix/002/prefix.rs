// Answer 0

#[test]
#[should_panic]
fn test_remove_base64_suffix_missing_prefix() {
    let test_input = "data:text/plain;base64,abcdeSAB;";
    let _result = remove_base64_suffix(test_input);
}

#[test]
#[should_panic]
fn test_remove_base64_suffix_invalid_suffix_character() {
    let test_input = "data:text/plain;base64,abcdfg;";
    let _result = remove_base64_suffix(test_input);
}

#[test]
#[should_panic]
fn test_remove_base64_suffix_no_suffix() {
    let test_input = "data:text/plain;";
    let _result = remove_base64_suffix(test_input);
}

#[test]
fn test_remove_base64_suffix_trailing_whitespace() {
    let test_input = "data:text/plain;base64,abcdeSAB;   ";
    let _result = remove_base64_suffix(test_input);
}

#[test]
fn test_remove_base64_suffix_only_invalid_characters() {
    let test_input = "data:text/plain;base64,123456;";
    let _result = remove_base64_suffix(test_input);
}

