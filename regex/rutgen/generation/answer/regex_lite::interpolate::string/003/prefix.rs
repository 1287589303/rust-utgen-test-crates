// Answer 0

#[test]
fn test_string_with_numbered_capture() {
    let mut dst = String::new();
    let replacement = "Hello $1!";
    let mut append = |i: usize, dst: &mut String| {
        if i == 1 {
            dst.push_str("World");
        }
    };
    let name_to_index = |_: &str| None; // No named captures in this test

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_named_capture() {
    let mut dst = String::new();
    let replacement = "Hello ${name}!";
    let mut append = |i: usize, dst: &mut String| {
        if i == 0 {
            dst.push_str("Universe");
        }
    };
    let name_to_index = |name: &str| {
        if name == "name" {
            Some(0)
        } else {
            None
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_string_with_multiple_captures() {
    let mut dst = String::new();
    let replacement = "$1 says hello to ${friend}!";
    let mut append = |i: usize, dst: &mut String| {
        if i == 1 {
            dst.push_str("Alice");
        } else if i == 0 {
            dst.push_str("Bob");
        }
    };
    let name_to_index = |name: &str| {
        if name == "friend" {
            Some(0)
        } else {
            None
        }
    };

    string(replacement, append, name_to_index, &mut dst);
}

