// Answer 0

#[test]
#[should_panic]
fn test_advance_with_length_zero() {
    let mut buffer: &[u8] = &[];
    buffer.advance(1);
}

#[test]
#[should_panic]
fn test_advance_with_length_one() {
    let mut buffer: &[u8] = &[1];
    buffer.advance(2);
}

#[test]
#[should_panic]
fn test_advance_with_length_two() {
    let mut buffer: &[u8] = &[1, 2];
    buffer.advance(3);
}

#[test]
#[should_panic]
fn test_advance_with_length_three() {
    let mut buffer: &[u8] = &[1, 2, 3];
    buffer.advance(4);
}

