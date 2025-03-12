// Answer 0

#[test]
fn test_parse_line_col_case_1() {
    let mut msg = String::from("Error occurred at line 10 column abc");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_2() {
    let mut msg = String::from("Parsing failed at line 20 column xyz");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_3() {
    let mut msg = String::from("Unexpected token at line 30 column end");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_case_4() {
    let mut msg = String::from("Syntax error at line 40 column ;");
    let result = parse_line_col(&mut msg);
}

