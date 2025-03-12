// Answer 0

#[test]
fn test_remove_base64_suffix_not_ending_with_4() {
    let input = "data:text/plain;base64,QmFzZTY0IHNjaGVtZXMgdGhlIHNlY3JldCBtZXRob2Q=";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_not_ending_with_4_with_tabs() {
    let input = "data:text/plain;base64,QmFzZTY0IHNjaGVtZXMgdGhlIHNlY3JldCBtZXRob2Q=\t";
    let result = remove_base64_suffix(input);
}

#[test]
fn test_remove_base64_suffix_not_ending_with_4_with_newlines() {
    let input = "data:text/plain;base64,QmFzZTY0IHNjaGVtZXMgdGhlIHNlY3JldCBtZXRob2Q=\n";
    let result = remove_base64_suffix(input);
}

