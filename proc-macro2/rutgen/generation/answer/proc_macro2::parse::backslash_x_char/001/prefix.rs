// Answer 0

#[test]
fn test_backslash_x_char_valid_sequence_1() {
    let input = vec![(0, '3'), (1, '7'), (2, 'A')].into_iter();
    let mut chars = input;
    let result = backslash_x_char(&mut chars);
}

#[test]
fn test_backslash_x_char_valid_sequence_2() {
    let input = vec![(0, '4'), (1, '9'), (2, 'b')].into_iter();
    let mut chars = input;
    let result = backslash_x_char(&mut chars);
}

#[test]
fn test_backslash_x_char_valid_sequence_3() {
    let input = vec![(0, '7'), (1, '0'), (2, 'f')].into_iter();
    let mut chars = input;
    let result = backslash_x_char(&mut chars);
}

#[test]
fn test_backslash_x_char_valid_sequence_4() {
    let input = vec![(0, '2'), (1, '5'), (2, 'C')].into_iter();
    let mut chars = input;
    let result = backslash_x_char(&mut chars);
}

#[test]
fn test_backslash_x_char_valid_sequence_5() {
    let input = vec![(0, '5'), (1, '3'), (2, 'a')].into_iter();
    let mut chars = input;
    let result = backslash_x_char(&mut chars);
}

