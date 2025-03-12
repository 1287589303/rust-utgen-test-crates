// Answer 0

#[test]
fn test_quit_byte_true_case_ascii() {
    let config = Config::new().unicode_word_boundary(false);
    let result = config.quit(b'a', true);
}

#[test]
fn test_quit_byte_true_case_non_ascii() {
    let config = Config::new().unicode_word_boundary(false);
    let result = config.quit(b'\xFF', true);
}

