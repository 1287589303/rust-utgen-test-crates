// Answer 0

#[test]
fn test_is_passthrough_ascii_label_case_1() {
    let label: &[u8] = &[b'a', b'b', b'-', b'c'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case_2() {
    let label: &[u8] = &[b'x', b'y', b'-', b'x'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case_3() {
    let label: &[u8] = &[b'z', b'y', b'-', b'z'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case_4() {
    let label: &[u8] = &[b'a', b'1', b'-', b'c'];
    let result = is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case_5() {
    let label: &[u8] = &[b'b', b'a', b'-', b'2'];
    let result = is_passthrough_ascii_label(label);
}

