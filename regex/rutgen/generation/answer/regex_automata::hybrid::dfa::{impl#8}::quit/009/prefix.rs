// Answer 0

#[test]
#[should_panic]
fn test_quit_non_ascii_when_unicode_boundary_true() {
    let config = Config::new().unicode_word_boundary(true);
    config.quit(128, false);
}

#[test]
#[should_panic]
fn test_quit_non_ascii_when_unicode_boundary_true_2() {
    let config = Config::new().unicode_word_boundary(true);
    config.quit(200, false);
}

#[test]
#[should_panic]
fn test_quit_non_ascii_when_unicode_boundary_true_3() {
    let config = Config::new().unicode_word_boundary(true);
    config.quit(255, false);
}

