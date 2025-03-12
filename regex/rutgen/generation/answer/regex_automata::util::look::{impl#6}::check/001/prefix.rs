// Answer 0

#[test]
#[cfg(not(feature = "unicode-word-boundary"))]
fn test_check_unicode_word_boundary_disabled() {
    let result = crate::util::check();
}

#[test]
#[cfg(feature = "unicode-word-boundary")]
fn test_check_unicode_word_boundary_enabled() {
    let result = crate::util::check();
}

