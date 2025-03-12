// Answer 0

#[test]
fn test_ascii_deny_list_new_empty() {
    let deny_list = "";
    let result = AsciiDenyList::new(false, deny_list);
}

#[test]
fn test_ascii_deny_list_new_single_permissible() {
    let deny_list = "!";
    let result = AsciiDenyList::new(false, deny_list);
}

#[test]
fn test_ascii_deny_list_new_max_valid() {
    let deny_list = "!".repeat(64);
    let result = AsciiDenyList::new(false, &deny_list);
}

#[should_panic]
#[test]
fn test_ascii_deny_list_new_exceeding_max() {
    let deny_list = "!".repeat(65);
    let result = AsciiDenyList::new(false, &deny_list);
}

