// Answer 0

#[test]
fn test_bytes_with_named_capture() {
    let mut dst = Vec::new();
    let replacement = b"foo $bar baz";

    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    };

    let name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_escaped_dollar() {
    let mut dst = Vec::new();
    let replacement = b"foo $$bar baz";

    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    };

    let name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_numeric_capture() {
    let mut dst = Vec::new();
    let replacement = b"foo $1 baz";

    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 1 {
            dst.extend_from_slice(b"ONE");
        }
    };

    let name_to_index = |name: &str| {
        None  // This will not be used in this particular case
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_multiple_replacements() {
    let mut dst = Vec::new();
    let replacement = b"$bar goes to $1";

    let append = |index: usize, dst: &mut Vec<u8>| {
        match index {
            0 => dst.extend_from_slice(b"BAR"),
            1 => dst.extend_from_slice(b"ONE"),
            _ => {}
        }
    };

    let name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

