// Answer 0

#[test]
fn test_bytes_single_named_capture() {
    let mut dst = Vec::new();
    let mut replacement = b"foo $bar baz".to_vec();
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    };
    let mut name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };
    bytes(&mut replacement, &mut append, &mut name_to_index, &mut dst);
}

#[test]
fn test_bytes_double_dollar_escape() {
    let mut dst = Vec::new();
    let mut replacement = b"foo $$bar baz".to_vec();
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    };
    let mut name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };
    bytes(&mut replacement, &mut append, &mut name_to_index, &mut dst);
}

#[test]
fn test_bytes_multiple_replacements() {
    let mut dst = Vec::new();
    let mut replacement = b"foo $bar baz $baz".to_vec();
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"BAR");
        }
    };
    let mut name_to_index = |name: &str| {
        if name == "bar" {
            Some(0)
        } else {
            None
        }
    };
    bytes(&mut replacement, &mut append, &mut name_to_index, &mut dst);
}

#[test]
fn test_bytes_no_capture_reference() {
    let mut dst = Vec::new();
    let mut replacement = b"foo baz".to_vec();
    let mut append = |index: usize, dst: &mut Vec<u8>| {
        // This should not be called since there's no capture reference
    };
    let mut name_to_index = |name: &str| {
        // This should not be called either
        None
    };
    bytes(&mut replacement, &mut append, &mut name_to_index, &mut dst);
}

