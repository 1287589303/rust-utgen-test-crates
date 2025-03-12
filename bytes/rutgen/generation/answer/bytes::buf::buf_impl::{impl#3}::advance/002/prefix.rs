// Answer 0

#[test]
fn test_advance_zero() {
    let data = vec![1, 2, 3, 4, 5];
    let mut cursor = std::io::Cursor::new(data);
    let initial_pos = cursor.position();
    cursor.advance(0);
    assert_eq!(initial_pos, cursor.position());
}

#[test]
fn test_advance_to_end() {
    let data = vec![1, 2, 3, 4, 5];
    let mut cursor = std::io::Cursor::new(data);
    let length = cursor.get_ref().as_ref().len() as u64;
    cursor.advance(length - cursor.position());
    assert_eq!(cursor.position(), length);
}

#[test]
fn test_advance_within_bounds() {
    let data = vec![1, 2, 3, 4, 5];
    let mut cursor = std::io::Cursor::new(data);
    let initial_pos = cursor.position();
    cursor.advance(3);
    assert_eq!(cursor.position(), initial_pos + 3);
}

