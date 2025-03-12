// Answer 0

#[test]
fn test_find_cap_ref_empty() {
    let replacement: &[u8] = &[];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_single_byte_no_dollar() {
    let replacement: &[u8] = &[b'a'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_first_byte_not_dollar() {
    let replacement: &[u8] = &[b'a', b'b', b'c'];
    let result = find_cap_ref(replacement);
}

