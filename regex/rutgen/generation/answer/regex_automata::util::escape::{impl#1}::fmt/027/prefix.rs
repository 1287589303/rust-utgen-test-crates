// Answer 0

#[test]
fn test_debug_haystack_with_control_characters() {
    let input = b"\x0bHello\x0cWorld\x00\x7f\n\r\t"; // contains '\x0b', '\x0c', '\0', '\x7f', '\n', '\r', '\t'
    let haystack = DebugHaystack(input);
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_with_only_control_characters() {
    let input = b"\x0b\x0c\x00\x7f\n\r\t"; // only control characters, meeting all preconditions
    let haystack = DebugHaystack(input);
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_with_multiple_control_characters() {
    let input = b"Sample\x0bText\x0c\x00End\x7f\n\r\t"; // includes all required control characters and a sample text
    let haystack = DebugHaystack(input);
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_with_single_control_characters() {
    let input = b"\x0b"; // single instance of '\x0b'
    let haystack = DebugHaystack(input);
    let _ = format!("{:?}", haystack);
}

#[test]
fn test_debug_haystack_with_combined_characters() {
    let input = b"\x0bText\x00More\x7f\n\rTabs\t"; // combines several valid characters with control character 
    let haystack = DebugHaystack(input);
    let _ = format!("{:?}", haystack);
}

