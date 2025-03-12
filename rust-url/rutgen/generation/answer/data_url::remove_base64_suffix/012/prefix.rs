// Answer 0

#[test]
fn test_remove_base64_suffix_invalid_suffix() {
    let input = "   some_random_data   1234   \n"; 
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_missing_e() {
    let input = "valid_data_42   jeSAb;   "; 
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_missing_s() {
    let input = "base64_data   64eXAb;  "; 
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_missing_6() {
    let input = "example_data  46eFAb;"; 
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_non_base64_characters() {
    let input = "d#ata@input   46eSAb;"; 
    let result = remove_base64_suffix(input);
}

