// Answer 0

#[test]
fn test_parse_line_col_no_suffix() {
    let mut msg = String::from("Error occurred without line information");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_empty_string() {
    let mut msg = String::from("");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_only_text() {
    let mut msg = String::from("Some error happened");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_invalid_format() {
    let mut msg = String::from("Incorrect error format at column 5");
    let result = parse_line_col(&mut msg);
}

#[test]
fn test_parse_line_col_no_line_info() {
    let mut msg = String::from("Error has occurred because of unknown reasons.");
    let result = parse_line_col(&mut msg);
}

