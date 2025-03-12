// Answer 0

#[test]
fn test_quit_with_non_ascii_byte_when_unicode_word_boundary_enabled() {
    let config = Config::new()
        .unicode_word_boundary(true)
        .quitset(Some(ByteSet::empty()));

    let non_ascii_byte: u8 = 128; // example non-ASCII byte
    let updated_config = config.quit(non_ascii_byte, true);
}

#[test]
#[should_panic]
fn test_quit_with_non_ascii_byte_when_unicode_word_boundary_enabled_with_removal() {
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quitset(Some(ByteSet::empty()));

    let non_ascii_byte: u8 = 128; // example non-ASCII byte
    config = config.quit(non_ascii_byte, true);
    let _updated_config = config.quit(non_ascii_byte, false); // should panic here
}

#[test]
fn test_quit_with_non_ascii_byte_when_unicode_word_boundary_enabled_and_quitset_exists() {
    let mut byte_set = ByteSet::empty();
    byte_set.add(128);  // initially add a non-ASCII byte

    let config = Config::new()
        .unicode_word_boundary(true)
        .quitset(Some(byte_set));

    let non_ascii_byte: u8 = 129; // another non-ASCII byte
    let updated_config = config.quit(non_ascii_byte, false);
}

