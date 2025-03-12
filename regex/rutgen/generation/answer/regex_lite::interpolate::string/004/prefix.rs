// Answer 0

#[test]
fn test_string_with_number_capture() {
    let mut dst = String::new();
    let replacement = "$0 test $1";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("first_capture");
        } else if i == 1 {
            dst.push_str("second_capture");
        }
    };
    let name_to_index = |name: &str| match name {
        "first" => Some(0),
        "second" => Some(1),
        _ => None,
    };
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_non_existent_capture() {
    let mut dst = String::new();
    let replacement = "$0 test $name";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("exists");
        }
    };
    let name_to_index = |_| None;
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_escaped_dollar() {
    let mut dst = String::new();
    let replacement = "$$0 should be escaped $1";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("value");
        } else if i == 1 {
            dst.push_str("another_value");
        }
    };
    let name_to_index = |_| None;
    string(replacement, append, name_to_index, &mut dst);
}

