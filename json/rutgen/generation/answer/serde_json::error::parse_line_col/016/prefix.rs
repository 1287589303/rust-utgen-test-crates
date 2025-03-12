// Answer 0

#[test]
fn test_parse_line_col_case_1() {
    let mut msg = String::from("Error occurred at line 42 column X");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_2() {
    let mut msg = String::from("Parsing failed at line 10 column ABC");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_3() {
    let mut msg = String::from("Unexpected EOF at line 7 column ABCDEF");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_4() {
    let mut msg = String::from("Syntax error at line 123 column INVALID");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_5() {
    let mut msg = String::from("Type mismatch at line 5 column ERROR_MSG");
    let result = parse_line_col(&mut msg);
}

