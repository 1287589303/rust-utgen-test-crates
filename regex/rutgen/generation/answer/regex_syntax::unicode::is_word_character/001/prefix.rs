// Answer 0

#[test]
fn test_is_word_character_valid_ascii() {
    let valid_chars = ['a', 'Z', '0', '_'];
    for &c in &valid_chars {
        let _ = is_word_character(c);
    }
}

#[test]
fn test_is_word_character_invalid_ascii() {
    let invalid_chars = [' ', '!', '@', '#'];
    for &c in &invalid_chars {
        let _ = is_word_character(c);
    }
}

#[test]
fn test_is_word_character_valid_unicode() {
    let valid_unicode_chars = ['é', 'ß', '你', '😀'];
    for &c in &valid_unicode_chars {
        let _ = is_word_character(c);
    }
}

#[test]
fn test_is_word_character_invalid_unicode() {
    let invalid_unicode_chars = ['\u{200B}', '\u{202E}', '\u{FFFF}'];
    for &c in &invalid_unicode_chars {
        let _ = is_word_character(c);
    }
}

#[test]
#[cfg(not(feature = "unicode-perl"))]
fn test_is_word_character_not_enabled() {
    let test_chars = ['a', '1', '!', ' '];
    for &c in &test_chars {
        let _ = is_word_character(c);
    }
}

#[test]
#[cfg(feature = "unicode-perl")]
fn test_is_word_character_enabled() {
    let test_chars = ['a', '1', '!', ' '];
    for &c in &test_chars {
        let _ = is_word_character(c);
    }
}

