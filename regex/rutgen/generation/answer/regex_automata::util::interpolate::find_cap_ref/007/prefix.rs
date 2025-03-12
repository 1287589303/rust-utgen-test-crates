// Answer 0

#[test]
fn test_find_cap_ref_short_input() {
    let replacement: &[u8] = b"$";
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_not_start_with_dollar() {
    let replacement: &[u8] = b"abc";
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_braced_start() {
    let replacement: &[u8] = b"${name}";
    let result = find_cap_ref(replacement);
}

#[test]
fn test_find_cap_ref_no_valid_cap_letter() {
    let replacement: &[u8] = b"$@";
    let result = find_cap_ref(replacement);
} 

#[test]
fn test_find_cap_ref_empty_after_dollar() {
    let replacement: &[u8] = b"${}";
    let result = find_cap_ref(replacement);
}

