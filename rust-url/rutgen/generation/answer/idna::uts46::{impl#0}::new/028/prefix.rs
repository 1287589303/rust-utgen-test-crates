// Answer 0

#[test]
#[should_panic]
fn test_new_with_invalid_lowercase_letter() {
    let deny_list = "a"; // will trigger panic for lowercase letter
    let result = AsciiDenyList::new(false, deny_list);
}

#[test]
#[should_panic]
fn test_new_with_invalid_lowercase_letters() {
    let deny_list = "abc"; // will trigger panic for lowercase letters
    let result = AsciiDenyList::new(false, deny_list);
}

#[test]
fn test_new_with_high_unicode() {
    let deny_list = "€"; // valid input, no disallowed ASCII chars, only Unicode
    let result = AsciiDenyList::new(false, deny_list);
}

#[test]
fn test_new_with_valid_high_unicode() {
    let deny_list = "あいうえお"; // valid input, no ASCII chars
    let result = AsciiDenyList::new(false, deny_list);
}

