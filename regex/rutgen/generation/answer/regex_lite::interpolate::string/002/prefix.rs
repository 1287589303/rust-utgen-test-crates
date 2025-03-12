// Answer 0

#[test]
fn test_string_with_single_number_capture() {
    let mut dst = String::new();
    let replacement = "$1";
    let mut capture_values = vec!["value1"];
    let append = |i: usize, dst: &mut String| {
        if i < capture_values.len() {
            dst.push_str(capture_values[i]);
        }
    };
    let name_to_index = |_name: &str| None; // No named captures in this case
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_named_capture() {
    let mut dst = String::new();
    let replacement = "${name}";
    let capture_values = vec!["value1"];
    let append = |i: usize, dst: &mut String| {
        if i < capture_values.len() {
            dst.push_str(capture_values[i]);
        }
    };
    let name_to_index = |name: &str| {
        if name == "name" { Some(0) } else { None }
    };
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_escaped_dollar() {
    let mut dst = String::new();
    let replacement = "$$1";
    let mut capture_values = vec!["value1"];
    let append = |i: usize, dst: &mut String| {
        if i < capture_values.len() {
            dst.push_str(capture_values[i]);
        }
    };
    let name_to_index = |_name: &str| None;
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_multiple_captures() {
    let mut dst = String::new();
    let replacement = "$1 and ${name}";
    let capture_values = vec!["value1", "value2"];
    let append = |i: usize, dst: &mut String| {
        if i < capture_values.len() {
            dst.push_str(capture_values[i]);
        }
    };
    let name_to_index = |name: &str| {
        if name == "name" { Some(0) } else { None }
    };
    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_no_captures() {
    let mut dst = String::new();
    let replacement = "No captures here.";
    let append = |_i: usize, _dst: &mut String| {}; // No captures to append
    let name_to_index = |_name: &str| None; // No named captures
    string(replacement, append, name_to_index, &mut dst);
}

