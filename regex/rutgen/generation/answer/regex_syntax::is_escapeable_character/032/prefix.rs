// Answer 0

#[test]
fn test_escapeable_character_less_than() {
    let c = '<';
    let _ = is_escapeable_character(c);
}

#[test]
fn test_escapeable_character_greater_than() {
    let c = '>';
    let _ = is_escapeable_character(c);
}

#[test]
fn test_escapeable_character_percent() {
    let c = '%';
    let _ = is_escapeable_character(c);
}

#[test]
fn test_escapeable_character_slash() {
    let c = '/';
    let _ = is_escapeable_character(c);
}

#[test]
fn test_escapeable_character_exclamation() {
    let c = '!';
    let _ = is_escapeable_character(c);
}

