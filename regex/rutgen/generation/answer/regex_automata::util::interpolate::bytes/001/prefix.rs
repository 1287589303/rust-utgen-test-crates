// Answer 0

#[test]
fn test_bytes_empty_replacement() {
    let mut dst = Vec::new();
    let replacement: &[u8] = b"";
    let append = |_, _: &mut Vec<u8>| {};
    let name_to_index = |_: &str| None;
    bytes(replacement, append, name_to_index, &mut dst);
}

#[test]
fn test_bytes_empty_replacement_with_non_empty_dst() {
    let mut dst = vec![b'x', b'y', b'z'];
    let replacement: &[u8] = b"";
    let append = |_, _: &mut Vec<u8>| {};
    let name_to_index = |_: &str| None;
    bytes(replacement, append, name_to_index, &mut dst);
}

