// Answer 0

#[test]
fn test_quit_byte_non_ascii_enabled() {
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quitset(Some(ByteSet::empty()));
    config.quit(128, true);
}

#[test]
#[should_panic] // Should panic because we try to remove a non-ASCII byte while unicode_word_boundary is enabled
fn test_quit_byte_non_ascii_remove() {
    let mut config = Config::new()
        .unicode_word_boundary(true)
        .quitset(Some(ByteSet::empty()));
    config.quit(128, true); // Adding a quit byte
    config.quit(128, false); // Attempting to remove it should panic
}

