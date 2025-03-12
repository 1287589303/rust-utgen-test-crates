// Answer 0

#[test]
fn test_remove_base64_suffix_case_1() {
    let input = "data  \tesa4some_other_characters_before_semicolon ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_2() {
    let input = "123   eS4randomtext;\n";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_3() {
    let input = "sample  \n  data esA4   trailing data ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_4() {
    let input = "value eSa4 several characters iterated     ";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_case_5() {
    let input = "  blah    eSa4   different patterns    ;";
    let result = remove_base64_suffix(input);
}

