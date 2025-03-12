// Answer 0

#[test]
fn test_find_cap_ref_empty_input() {
    let replacement: &[u8] = &[b'a'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_dollar_only() {
    let replacement: &[u8] = &[b'$'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_empty_braces() {
    let replacement: &[u8] = &[b'$', b'{', b'}'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_number_capture() {
    let replacement: &[u8] = &[b'$', b'1'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_zero_capture() {
    let replacement: &[u8] = &[b'$', b'0', b'a', b'b', b'c'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_named_capture_with_empty_braces() {
    let replacement: &[u8] = &[b'$', b'a', b'b', b'c', b'{', b'}'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_named_capture_with_braces() {
    let replacement: &[u8] = &[b'$', b'a', b'b', b'c', b'{', b'd', b'e', b'f', b'}'];
    find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_incomplete_braces() {
    let replacement: &[u8] = &[b'$', b'a', b'b', b'c', b'{'];
    find_cap_ref(replacement);
}

