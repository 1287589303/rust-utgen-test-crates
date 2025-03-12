// Answer 0

#[test]
fn test_valid_data_url_with_space_prefix() {
    let result = pretend_parse_data_url("   data:hello world!");
}

#[test]
fn test_valid_data_url_with_tabs_and_newlines() {
    let result = pretend_parse_data_url("data:\n\ttext");
}

#[test]
fn test_valid_data_url_mixed_case() {
    let result = pretend_parse_data_url("Data: 123");
}

#[test]
fn test_valid_data_url_with_control_chars() {
    let result = pretend_parse_data_url("\r\n\tdata:abc ");
}

#[test]
fn test_valid_data_url_case_variation() {
    let result = pretend_parse_data_url("DaTa:hello world!");
}

#[test]
fn test_valid_data_url_with_extra_spaces() {
    let result = pretend_parse_data_url("data:     ");
}

