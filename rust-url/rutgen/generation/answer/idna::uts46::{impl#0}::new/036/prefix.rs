// Answer 0

#[test]
#[should_panic]
fn test_ascii_deny_list_with_dot() {
    let _deny_list = AsciiDenyList::new(false, "abc.def");
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_dot_using_extended_characters() {
    let _deny_list = AsciiDenyList::new(false, "def..");
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_dot_at_start() {
    let _deny_list = AsciiDenyList::new(false, ".ghi");
}

#[test]
#[should_panic]
fn test_ascii_deny_list_with_dot_at_end() {
    let _deny_list = AsciiDenyList::new(false, "jkl.");
}

