// Answer 0

#[test]
fn test_ascii_deny_list_with_digits() {
    let deny_glyphless = false;
    let deny_list = "0123456789";
    let deny_list_result = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic(expected = "ASCII deny list must not contain digits.")]
fn test_ascii_deny_list_with_digit_zero() {
    let deny_glyphless = false;
    let deny_list = "0";
    let deny_list_result = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic(expected = "ASCII deny list must not contain digits.")]
fn test_ascii_deny_list_with_digit_nine() {
    let deny_glyphless = false;
    let deny_list = "9";
    let deny_list_result = AsciiDenyList::new(deny_glyphless, deny_list);
}

#[test]
#[should_panic(expected = "ASCII deny list must not contain digits.")]
fn test_ascii_deny_list_with_all_digits() {
    let deny_glyphless = false;
    let deny_list = "0123456789";
    let deny_list_result = AsciiDenyList::new(deny_glyphless, deny_list);
}

