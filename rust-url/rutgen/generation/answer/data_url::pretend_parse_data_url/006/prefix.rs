// Answer 0

#[test]
fn test_pretend_parse_data_url_valid_case() {
    let input = "data:example data";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_with_leading_spaces() {
    let input = "   data:another_example";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_with_tabs() {
    let input = "data:\texample\tdata";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_with_newlines() {
    let input = "data:\nnew\nline data";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_with_spaces_after_colon() {
    let input = "data:   trailing spaces    ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_case_insensitive() {
    let input = "DATA:caseInsensitiveExample";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_empty_after_colon() {
    let input = "data:";
    pretend_parse_data_url(input);
}

