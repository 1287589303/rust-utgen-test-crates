// Answer 0

#[test]
#[should_panic]
fn test_quit_non_ascii_byte_when_unicode_word_boundary_enabled() {
    let mut config = Config::new()
        .unicode_word_boundary(true); // precondition: self.get_unicode_word_boundary() is true
    
    let non_ascii_byte = 128; // a non-ASCII byte

    config.quit(non_ascii_byte, false); // precondition: byte.is_ascii() is false, yes is false
}

#[test]
#[should_panic]
fn test_quit_another_non_ascii_byte_when_unicode_word_boundary_enabled() {
    let mut config = Config::new()
        .unicode_word_boundary(true); // precondition: self.get_unicode_word_boundary() is true
    
    let non_ascii_byte = 200; // another non-ASCII byte

    config.quit(non_ascii_byte, false); // precondition: byte.is_ascii() is false, yes is false
}

