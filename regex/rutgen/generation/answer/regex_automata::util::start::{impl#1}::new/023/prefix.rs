// Answer 0

#[test]
fn test_new_with_exceeding_byte_and_line_terminator_nl() {
    let mut look_matcher = LookMatcher::new();
    look_matcher.set_line_terminator(b'\n');
    let start_byte_map = StartByteMap::new(&look_matcher);
}

#[test]
fn test_new_with_exceeding_byte_and_line_terminator_custom() {
    let mut look_matcher = LookMatcher::new();
    look_matcher.set_line_terminator(b'\x1E'); // a custom line terminator
    let start_byte_map = StartByteMap::new(&look_matcher);
}

#[test]
fn test_new_with_exceeding_byte_and_line_terminator_crlf() {
    let mut look_matcher = LookMatcher::new();
    look_matcher.set_line_terminator(b'\r'); // ensure to set CR to see mapping
    let start_byte_map = StartByteMap::new(&look_matcher);
}

