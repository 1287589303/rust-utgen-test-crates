// Answer 0

#[test]
#[should_panic]
fn test_backslash_x_byte_invalid_first_character() {
    let input = vec![(0, b'g'), (1, b'1')].into_iter();
    let mut chars = input;
    let _ = backslash_x_byte(&mut chars);
}

#[test]
#[should_panic]
fn test_backslash_x_byte_invalid_second_character() {
    let input = vec![(0, b'1'), (1, b'z')].into_iter();
    let mut chars = input;
    let _ = backslash_x_byte(&mut chars);
}

#[test]
#[should_panic]
fn test_backslash_x_byte_invalid_both_characters() {
    let input = vec![(0, b'z'), (1, b'y')].into_iter();
    let mut chars = input;
    let _ = backslash_x_byte(&mut chars);
}

