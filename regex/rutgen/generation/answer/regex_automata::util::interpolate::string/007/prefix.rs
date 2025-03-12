// Answer 0

#[test]
fn test_string_with_no_cap_ref() {
    let mut dst = String::new();
    let replacement = "$bar baz";

    let append = |index: usize, dst: &mut String| {
        if index == 0 {
            dst.push_str("BAR");
        }
    };

    let name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_empty_replacement() {
    let mut dst = String::new();
    let replacement = "";

    let append = |index: usize, dst: &mut String| {
        // Nothing to do since the replacement string is empty
    };

    let name_to_index = |name: &str| {
        None
    };

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_non_matching_name() {
    let mut dst = String::new();
    let replacement = "$unknown baz";

    let append = |index: usize, dst: &mut String| {
        // Append nothing if index is invalid
    };

    let name_to_index = |name: &str| {
        if name == "known" {
            Some(0)
        } else {
            None
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

