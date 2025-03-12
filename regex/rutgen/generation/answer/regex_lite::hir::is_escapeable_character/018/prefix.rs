// Answer 0

#[test]
fn test_is_escapeable_character_with_backslash() {
    let c = '\\';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_with_dollar() {
    let c = '$';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_with_percent() {
    let c = '%';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_with_ampersand() {
    let c = '&';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_with_hyphen() {
    let c = '-';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_with_tilde() {
    let c = '~';
    let result = is_escapeable_character(c);
}

