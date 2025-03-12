// Answer 0

#[test]
fn test_empty_string() {
    pretend_parse_data_url("")
}

#[test]
fn test_whitespace_string() {
    pretend_parse_data_url("    ")
}

#[test]
fn test_tab_string() {
    pretend_parse_data_url("\t\t\t")
}

#[test]
fn test_newline_string() {
    pretend_parse_data_url("\n\n\n")
}

#[test]
fn test_invalid_prefix() {
    pretend_parse_data_url("invalid_data_url")
}

#[test]
fn test_mixed_case_invalid_prefix() {
    pretend_parse_data_url("Invalid_Data_URL")
}

#[test]
fn test_non_data_prefix() {
    pretend_parse_data_url("text/plain;base64,SGVsbG8sIFdvcmxkIQ==")
}

#[test]
fn test_invalid_data_url_with_extra_spaces() {
    pretend_parse_data_url("      not_a_data_url     ")
}

