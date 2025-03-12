// Answer 0

#[test]
fn test_string_with_number_capture() {
    let mut dst = String::new();
    let replacement = "$0";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("value_for_0");
        }
    };
    let name_to_index = |_| None;
    
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_named_capture() {
    let mut dst = String::new();
    let replacement = "$name";
    let mut append = |i: usize, dst: &mut String| {
        if i == 1 {
            dst.push_str("value_for_name");
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
fn test_string_with_mixed_capture() {
    let mut dst = String::new();
    let replacement = "$0$other$name";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("value_for_0");
        } else if i == 1 {
            dst.push_str("value_for_name");
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
fn test_string_with_escaped_dollar() {
    let mut dst = String::new();
    let replacement = "$$0$name";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("value_for_0");
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

