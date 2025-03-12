// Answer 0

#[test]
fn test_parse_line_col_non_numeric_line() {
    let mut msg = String::from("Error occurred at line abc column 0");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_non_numeric_line_with_extra_spaces() {
    let mut msg = String::from("Error occurred at line    abc column 5");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_non_numeric_line_column_end() {
    let mut msg = String::from("Unexpected token at line xyz column ");
    let result = parse_line_col(&mut msg);
}

