// Answer 0

#[test]
fn test_is_word_byte_digit() {
    for b in 48..=57 {
        let _ = is_word_byte(b);
    }
}

#[test]
fn test_is_word_byte_uppercase() {
    for b in 65..=90 {
        let _ = is_word_byte(b);
    }
}

#[test]
fn test_is_word_byte_lowercase() {
    for b in 97..=122 {
        let _ = is_word_byte(b);
    }
}

#[test]
fn test_is_word_byte_underscore() {
    let _ = is_word_byte(b'_');
}

#[test]
fn test_is_word_byte_boundary_cases() {
    let _ = is_word_byte(0);
    let _ = is_word_byte(47);
    let _ = is_word_byte(58);
    let _ = is_word_byte(64);
    let _ = is_word_byte(91);
    let _ = is_word_byte(96);
    let _ = is_word_byte(123);
    let _ = is_word_byte(255);
}

