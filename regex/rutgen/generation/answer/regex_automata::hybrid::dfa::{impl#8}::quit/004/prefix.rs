// Answer 0

#[test]
fn test_quit_byte_removal() {
    let byte = b'a'; // ASCII byte
    let mut config = Config::new()
        .unicode_word_boundary(true) // precondition: self.get_unicode_word_boundary() is true
        .quit(b'a', true); // add a quit byte first
    
    // Now, the quitset is no longer none due to the previous call
    assert!(config.quitset.is_some()); // precondition: self.quitset.is_none() is false

    // Now remove the quit byte, which is allowed under the provided conditions
    let result = config.quit(byte, false); // precondition: yes is false

    // Result should be the same instance of Config
    assert!(result.get_quit(byte) == false); // Confirm the quit byte has been removed
}

#[test]
#[should_panic]
fn test_quit_byte_removal_panics_non_ascii() {
    let byte = 0xFF; // Non-ASCII byte
    let mut config = Config::new()
        .unicode_word_boundary(true) // precondition: self.get_unicode_word_boundary() is true
        .quit(b'a', true); // add a quit byte first

    // Now, the quitset is no longer none due to the previous call
    assert!(config.quitset.is_some()); // precondition: self.quitset.is_none() is false

    // Attempt to remove a non-ASCII byte while unicode word boundaries are enabled
    let _ = config.quit(byte, false); // This should panic
}

