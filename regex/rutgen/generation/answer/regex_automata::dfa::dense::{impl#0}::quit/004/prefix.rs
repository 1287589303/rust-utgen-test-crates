// Answer 0

#[test]
fn test_quit_method_with_ascii_byte() {
    let byte = b'a'; // ASCII byte
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quit(b'a', true); // initialize quitset

    config = config.quit(byte, false);
}

#[test]
#[should_panic]
fn test_quit_method_with_non_ascii_byte() {
    let byte = 200; // Non-ASCII byte
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quit(b'a', true); // initialize quitset

    config.quit(byte, false);
}

#[test]
fn test_quit_method_removing_quit_byte() {
    let byte = b'b'; // ASCII byte
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quit(b'b', true); // initialize quitset

    config = config.quit(byte, false);
}

#[test]
fn test_quit_method_setting_another_quit_byte() {
    let byte1 = b'c'; // ASCII byte
    let byte2 = b'd'; // Another ASCII byte
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quit(byte1, true); // initialize quitset

    config = config.quit(byte2, true);
}

