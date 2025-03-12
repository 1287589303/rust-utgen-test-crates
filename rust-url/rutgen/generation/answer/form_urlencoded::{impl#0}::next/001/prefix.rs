// Answer 0

#[test]
fn test_next_empty_input() {
    let input: &[u8] = &[];
    let mut parser = Parse { input };
    let result = parser.next();
    assert!(result.is_none());
}

