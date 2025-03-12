// Answer 0

#[test]
fn test_find_cap_ref_braced_no_closing_brace() {
    let rep: &[u8] = &[b'{', b'f', b'o', b'o', b'1', b'}', b'o', b't', b'h', b'e', b'r', b' ', b'd', b'a', b't', b'a'];
    let i: usize = 5;
    let result = find_cap_ref_braced(rep, i);
}

#[test]
fn test_find_cap_ref_braced_invalid_utf8() {
    let rep: &[u8] = &[b'{', b'\xff', b'\xfe', b'\xfd', b'}'];
    let i: usize = 4;
    let result = find_cap_ref_braced(rep, i);
}

