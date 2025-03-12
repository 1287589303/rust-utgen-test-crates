// Answer 0

#[test]
fn test_from_null_character() {
    let ch = '\u{0000}';
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_character_a() {
    let ch = 'a';
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_character_z() {
    let ch = 'z';
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_character_digit() {
    let ch = '5';
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_space_character() {
    let ch = ' ';
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_special_character() {
    let ch = '!';
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_unicode_character() {
    let ch = '\u{00E9}'; // é
    let _literal = Literal::from(ch);
}

#[test]
fn test_from_high_unicode_character() {
    let ch = '\u{10FFFF}';
    let _literal = Literal::from(ch);
}

