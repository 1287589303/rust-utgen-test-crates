// Answer 0

#[test]
fn test_parse_line_col_invalid_line_not_digit() {
    let mut msg = String::from("Error occurred at line A but no column info");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_column_not_starting() {
    let mut msg = String::from("Error occurred at line 5 but missing column info");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_column_no_suffix() {
    let mut msg = String::from("Error occurred at line 10 column");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_empty_string() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_partial_match() {
    let mut msg = String::from("An error occurred at line 8 and something else");
    let result = parse_line_col(&mut msg);
}

