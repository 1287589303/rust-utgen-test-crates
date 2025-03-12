// Answer 0

#[test]
fn test_is_passthrough_ascii_label_case1() {
    let label: &[u8] = &[b'a', b'B', b'1', b'C', b'@'];
    let _ = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case2() {
    let label: &[u8] = &[b'a', b'B', b'2', b'F', b'&'];
    let _ = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case3() {
    let label: &[u8] = &[b'a', b'B', b'3', b'G', b' '];
    let _ = is_passthrough_ascii_label(label);
}

