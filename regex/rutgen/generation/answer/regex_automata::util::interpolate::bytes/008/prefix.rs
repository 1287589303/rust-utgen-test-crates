// Answer 0

#[test]
fn test_bytes_no_capture_references() {
    let mut dst = vec![];
    let replacement: &[u8] = b"foo bar baz";

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
fn test_bytes_escaped_dollar() {
    let mut dst = vec![];
    let replacement: &[u8] = b"foo $$ bar baz";

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
fn test_bytes_multiple_non_capture_references() {
    let mut dst = vec![];
    let replacement: &[u8] = b"abc def ghi";

    let append = |index: usize, dst: &mut Vec<u8>| {
        // No captures to append in this test
    };

    let name_to_index = |name: &str| {
        // No names to map in this test
        None
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_mixed_references() {
    let mut dst = vec![];
    let replacement: &[u8] = b"hello world";

    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 1 {
            dst.extend_from_slice(b"WORLD");
        }
    };

    let name_to_index = |name: &str| {
        if name == "world" {
            Some(1)
        } else {
            None
        }
    };

    bytes(replacement, append, name_to_index, &mut dst);
}

