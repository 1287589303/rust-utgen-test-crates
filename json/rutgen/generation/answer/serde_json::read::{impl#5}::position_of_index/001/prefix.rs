// Answer 0

#[test]
fn test_position_of_index_multiple_newlines() {
    let data: &[u8] = b"First line\nSecond line\nThird line\n";
    let mut reader = SliceRead::new(data);
    let index = 20; // Point to a character in the "Second line"
    let position = reader.position_of_index(index);
}

#[test]
fn test_position_of_index_single_newline() {
    let data: &[u8] = b"Hello world\nGoodbye!";
    let mut reader = SliceRead::new(data);
    let index = 12; // First character of "Goodbye!"
    let position = reader.position_of_index(index);
}

#[test]
fn test_position_of_index_last_character() {
    let data: &[u8] = b"Line one\nLine two\n";
    let mut reader = SliceRead::new(data);
    let index = 16; // Last character after the second newline
    let position = reader.position_of_index(index);
}

#[test]
fn test_position_of_index_only_newline() {
    let data: &[u8] = b"\n";
    let mut reader = SliceRead::new(data);
    let index = 1; // Index after the newline
    let position = reader.position_of_index(index);
}

