// Answer 0

#[test]
fn test_next_with_empty_sequence() {
    let input: &[u8] = b"key1=value1&key2=&key3=value3";
    let mut parser = Parse { input };

    let _ = parser.next(); // "key1=value1" should be processed
    let _ = parser.next(); // "key2=" should result in empty sequence
    let result = parser.next(); // Subsequent call should yield None due to empty input

    assert_eq!(result, None);
}

#[test]
fn test_next_with_empty_input() {
    let input: &[u8] = b"&=&";
    let mut parser = Parse { input };

    let _ = parser.next(); // First call should process "&=" resulting in empty sequence
    let result = parser.next(); // Second call should yield None due to no more input

    assert_eq!(result, None);
}

#[test]
fn test_next_with_no_key_value_pairs() {
    let input: &[u8] = b"&&";
    let mut parser = Parse { input };

    let result = parser.next(); // No key-value pairs should lead to None due to empty input

    assert_eq!(result, None);
}

