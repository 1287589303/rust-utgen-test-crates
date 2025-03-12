// Answer 0

#[test]
fn test_string_with_number_capture() {
    let mut dst = String::new();
    let replacement = "foo $0 baz";
    let mut append = |index: usize, dst: &mut String| {
        if index == 0 {
            dst.push_str("BAR");
        }
    };
    let mut name_to_index = |_name: &str| None;
    
    crate::util::interpolate::string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_named_capture() {
    let mut dst = String::new();
    let replacement = "foo ${name} baz";
    let mut append = |_index: usize, _dst: &mut String| {};
    let mut name_to_index = |name: &str| {
        if name == "name" {
            Some(0)
        } else {
            None
        }
    };

    crate::util::interpolate::string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_multiple_captures() {
    let mut dst = String::new();
    let replacement = "start $0 middle $name end";
    let mut append = |index: usize, dst: &mut String| {
        if index == 0 {
            dst.push_str("BEGIN");
        }
    };
    let mut name_to_index = |name: &str| {
        if name == "name" {
            Some(1)
        } else {
            None
        }
    };

    crate::util::interpolate::string(replacement, append, name_to_index, &mut dst);
}

