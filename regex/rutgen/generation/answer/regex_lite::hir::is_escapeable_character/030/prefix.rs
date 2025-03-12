// Answer 0

#[test]
fn test_is_escapeable_character_percent() {
    let c = '%';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_caret() {
    let c = '^';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_ampersand() {
    let c = '&';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_hyphen() {
    let c = '-';
    let result = is_escapeable_character(c);
}

#[test]
fn test_is_escapeable_character_tilde() {
    let c = '~';
    let result = is_escapeable_character(c);
}

