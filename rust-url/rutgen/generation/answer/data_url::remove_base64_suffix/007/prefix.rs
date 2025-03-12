// Answer 0

#[test]
fn test_remove_base64_suffix_case_1() {
    let input = "data:text/plain;base64   ;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_2() {
    let input = "data:text/plain;base64\n";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_3() {
    let input = "data:text/plain;base64\t \r ;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_4() {
    let input = "data:text/plain;base64 ;";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_5() {
    let input = "data:text/plain;base64\r\n;";
    let result = remove_base64_suffix(input);
}

