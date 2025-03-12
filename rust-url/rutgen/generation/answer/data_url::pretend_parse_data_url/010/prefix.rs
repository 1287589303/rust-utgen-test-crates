// Answer 0

#[test]
fn test_pretend_parse_data_url_input_not_starting_with_data() {
    let input = "image/png, base64, iVBORw0KGgoAAAANSUhEUgAA...";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_leading_whitespace() {
    let input = "    data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA...";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_second_character_not_a() {
    let input = "data:x/png;base64,iVBORw0KGgoAAAANSUhEUgAA...";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_only_whitespace() {
    let input = "       ";
    pretend_parse_data_url(input);
}

#[test]
fn test_pretend_parse_data_url_control_characters_only() {
    let input = "\t\n\r  ";
    pretend_parse_data_url(input);
}

