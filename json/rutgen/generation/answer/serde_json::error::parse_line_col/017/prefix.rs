// Answer 0

#[test]
fn test_parse_line_col_valid_input() {
    let mut msg = String::from("Error occurred at line 10 column 5");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_boundary_case_line() {
    let mut msg = String::from("Error occurred at line 0 column 1");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_boundary_case_column() {
    let mut msg = String::from("Error occurred at line 10 column 0");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_large_numbers() {
    let mut msg = String::from("Error occurred at line 123456789 column 987654321");
    let result = parse_line_col(&mut msg);
}

