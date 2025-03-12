// Answer 0

#[test]
fn test_is_meta_character_non_meta_char() {
    let input = 'a';
    let _ = is_meta_character(input);
}

#[test]
fn test_is_meta_character_numeric() {
    let input = '5';
    let _ = is_meta_character(input);
}

#[test]
fn test_is_meta_character_space() {
    let input = ' ';
    let _ = is_meta_character(input);
}

#[test]
fn test_is_meta_character_special() {
    let input = '%';
    let _ = is_meta_character(input);
}

#[test]
fn test_is_meta_character_control_char() {
    let input = '\n';
    let _ = is_meta_character(input);
}

