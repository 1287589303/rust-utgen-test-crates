// Answer 0

#[test]
fn test_quit_with_min_byte_and_min_offset() {
    let byte = 0;
    let offset = 0;
    let _error = MatchError::quit(byte, offset);
}

#[test]
fn test_quit_with_min_byte_and_max_offset() {
    let byte = 0;
    let offset = usize::MAX;
    let _error = MatchError::quit(byte, offset);
}

#[test]
fn test_quit_with_max_byte_and_min_offset() {
    let byte = 255;
    let offset = 0;
    let _error = MatchError::quit(byte, offset);
}

#[test]
fn test_quit_with_max_byte_and_max_offset() {
    let byte = 255;
    let offset = usize::MAX;
    let _error = MatchError::quit(byte, offset);
}

#[test]
fn test_quit_with_middle_byte_and_middle_offset() {
    let byte = 128;
    let offset = 12345;
    let _error = MatchError::quit(byte, offset);
}

