// Answer 0

#[test]
fn test_bytes_with_number_capture() {
    let mut dst = Vec::new();
    let replacement: &[u8] = b"foo $1 bar";
    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 1 {
            dst.extend_from_slice(b"NUMBER");
        }
    };
    let name_to_index = |name: &str| -> Option<usize> {
        None
    };
    
    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_named_capture() {
    let mut dst = Vec::new();
    let replacement: &[u8] = b"foo $bar baz";
    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"NAMED");
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
fn test_bytes_with_multiple_captures() {
    let mut dst = Vec::new();
    let replacement: &[u8] = b"$1 and $bar";
    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 1 {
            dst.extend_from_slice(b"NUMBER");
        }
    };
    let name_to_index = |name: &str| {
        if name == "bar" {
            Some(1)
        } else {
            None
        }
    };
    
    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_with_escaped_dollar() {
    let mut dst = Vec::new();
    let replacement: &[u8] = b"foo $$bar baz";
    let append = |index: usize, dst: &mut Vec<u8>| {
        if index == 0 {
            dst.extend_from_slice(b"NAMED");
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

