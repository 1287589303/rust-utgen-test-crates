// Answer 0

#[test]
fn test_pretend_parse_data_url_case_insensitive_prefix() {
    let input = "   DaTa: example data";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_non_data_prefix() {
    let input = "   data1: example data";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_non_data_prefix_with_spaces() {
    let input = "   Data: ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_empty_string() {
    let input = "";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_only_whitespace() {
    let input = "   ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_spaces_before_and_after() {
    let input = "   data:     ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_str_with_valid_data_after_colon() {
    let input = "   DaTa: valid data";
    pretend_parse_data_url(input);
}

