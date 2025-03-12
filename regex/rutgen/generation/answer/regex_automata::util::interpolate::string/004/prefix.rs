// Answer 0

#[test]
fn test_string_with_number_capture() {
    let mut dst = String::new();
    let mut replacement = "Hello $0 World";

    let append = |index: usize, dst: &mut String| {
        if index == 0 {
            dst.push_str("CapturedValue");
        }
    };

    let name_to_index = |name: &str| {
        if name == "0" {
            Some(0)
        } else {
            None
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_named_capture() {
    let mut dst = String::new();
    let mut replacement = "Hello ${name} World";

    let append = |index: usize, dst: &mut String| {
        if index == 1 {
            dst.push_str("CapturedName");
        }
    };

    let name_to_index = |name: &str| {
        if name == "name" {
            Some(1)
        } else {
            None
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_mixed_captures() {
    let mut dst = String::new();
    let mut replacement = "Value: $0 and Name: ${name}";

    let append = |index: usize, dst: &mut String| {
        if index == 0 {
            dst.push_str("ValueCaptured");
        } else if index == 1 {
            dst.push_str("NameCaptured");
        }
    };

    let name_to_index = |name: &str| {
        match name {
            "0" => Some(0),
            "name" => Some(1),
            _ => None,
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_no_capture() {
    let mut dst = String::new();
    let mut replacement = "Hello World";

    let append = |index: usize, dst: &mut String| {
        dst.push_str("ShouldNotAppend");
    };

    let name_to_index = |_name: &str| None;

    string(replacement, append, name_to_index, &mut dst);
}

