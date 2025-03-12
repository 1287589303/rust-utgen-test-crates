// Answer 0

#[test]
fn test_is_passthrough_ascii_label_case1() {
    let label: &[u8] = b"azxy"; // label length is 4, does not contain hyphen
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case2() {
    let label: &[u8] = b"bcar"; // label length is 4, does not contain hyphen
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case3() {
    let label: &[u8] = b"xzyx"; // label length is 4, does not contain hyphen
    is_passthrough_ascii_label(label);
}

#[test]
fn test_is_passthrough_ascii_label_case4() {
    let label: &[u8] = b"dmne"; // label length is 4, does not contain hyphen
    is_passthrough_ascii_label(label);
}

