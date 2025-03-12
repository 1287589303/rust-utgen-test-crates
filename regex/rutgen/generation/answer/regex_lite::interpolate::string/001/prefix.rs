// Answer 0

#[test]
fn test_string_empty_replacement() {
    let replacement = "";
    let mut dst = String::new();
    let mut append = |i: usize, dst: &mut String| {
        dst.push_str(&i.to_string());
    };
    let mut name_to_index = |name: &str| {
        if name == "group" {
            Some(0)
        } else {
            None
        }
    };
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_empty_replacement_with_invalid_name() {
    let replacement = "";
    let mut dst = String::new();
    let mut append = |i: usize, dst: &mut String| {
        dst.push_str(&i.to_string());
    };
    let mut name_to_index = |name: &str| {
        if name == "invalid" {
            Some(0)
        } else {
            None
        }
    };
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_empty_replacement_with_no_capture_groups() {
    let replacement = "This is a test string.";
    let mut dst = String::new();
    let mut append = |i: usize, dst: &mut String| {
        dst.push_str(&i.to_string());
    };
    let mut name_to_index = |name: &str| {
        None
    };
    string(replacement, append, name_to_index, &mut dst);
}

