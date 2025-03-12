// Answer 0

#[test]
fn test_pretend_parse_data_url_valid_case() {
    let input = "  data:example_data  ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_only_whitespace() {
    let input = "    ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_valid_with_tabs() {
    let input = "\tdatacontent";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_valid_with_newlines() {
    let input = "\ndata:some\ncontent";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_no_data_prefix() {
    let input = "notdata:butthis";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_just_data_prefix() {
    let input = "data:";
    pretend_parse_data_url(input);
}

