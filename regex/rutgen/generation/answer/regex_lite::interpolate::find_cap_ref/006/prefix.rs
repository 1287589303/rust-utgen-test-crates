// Answer 0

#[test]
fn test_find_cap_ref_valid_number() {
    let replacement: &[u8] = &[b'$', b'1'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_valid_named() {
    let replacement: &[u8] = &[b'$', b'{', b'a', b'b', b'}'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_invalid_capture() {
    let replacement: &[u8] = &[b'$', b'{', b'a', b'b', b'}', b'c'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_length_one_invalid() {
    let replacement: &[u8] = &[b'$'];
    let result = find_cap_ref(replacement);
}

