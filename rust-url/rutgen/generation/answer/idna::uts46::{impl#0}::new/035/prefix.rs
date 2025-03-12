// Answer 0

#[test]
#[should_panic]
fn test_ascii_deny_list_with_hyphen() {
    let deny_glyphless = false;
    let deny_list = "-"; // Hyphen included, should panic
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_hyphen_and_other_valid_chars() {
    let deny_glyphless = false;
    let deny_list = "-!@#$%^&*()"; // Hyphen included, should panic
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

