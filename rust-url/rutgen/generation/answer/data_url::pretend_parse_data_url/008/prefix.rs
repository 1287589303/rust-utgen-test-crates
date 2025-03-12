// Answer 0

#[test]
fn test_invalid_data_url_missing_a() {
    let input = "data";
    pretend_parse_data_url(input);
}

#[test]
fn test_invalid_data_url_missing_t() {
    let input = "data:";
    pretend_parse_data_url(input);
}

#[test]
fn test_invalid_data_url_spaces_in_data() {
    let input = "dat a:";
    pretend_parse_data_url(input);
}

#[test]
fn test_invalid_data_url_spaces_between_chars() {
    let input = "da ta:";
    pretend_parse_data_url(input);
}

#[test]
fn test_invalid_data_url_spaces_after_first_char() {
    let input = "d ata:";
    pretend_parse_data_url(input);
}

#[test]
fn test_invalid_data_url_only_colon() {
    let input = "data:";
    pretend_parse_data_url(input);
}

