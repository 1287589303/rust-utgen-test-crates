// Answer 0

#[test]
#[should_panic]
fn test_ascii_deny_list_with_digit() {
    let deny_glyphless = false;
    let deny_list = "0"; // contains a digit
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_lowercase_letter() {
    let deny_glyphless = false;
    let deny_list = "a"; // contains a lowercase letter
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_uppercase_letter() {
    let deny_glyphless = false;
    let deny_list = "A"; // contains an uppercase letter
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_hyphen() {
    let deny_glyphless = false;
    let deny_list = "-"; // contains a hyphen
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_dot() {
    let deny_glyphless = false;
    let deny_list = "."; // contains a dot
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_multiple_prohibited_characters() {
    let deny_glyphless = false;
    let deny_list = "a0-.A"; // contains a lowercase letter, a digit, a hyphen, and a dot
    let _deny_list = AsciiDenyList::new(deny_glyphless, deny_list);
}

