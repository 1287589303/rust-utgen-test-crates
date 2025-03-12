// Answer 0

#[test]
fn test_find_cap_ref_valid_braced_non_number() {
    let replacement: &[u8] = &[b'$', b'{', b'a', b'b', b'}'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_valid_braced_mixed_chars() {
    let replacement: &[u8] = &[b'$', b'{', b'c', b'h', b'a', b'r', b'a', b'c', b't', b'e', b'r', b'}'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_valid_braced_special_chars() {
    let replacement: &[u8] = &[b'$', b'{', b'!', b'@', b'#', b'}'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_valid_braced_numeric_start() {
    let replacement: &[u8] = &[b'$', b'{', b'1', b'a', b'}'];
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_valid_braced_non_ascii() {
    let replacement: &[u8] = &[b'$', b'{', b'\x80', b'$' , b'}'];
    let result = find_cap_ref(replacement);
}

