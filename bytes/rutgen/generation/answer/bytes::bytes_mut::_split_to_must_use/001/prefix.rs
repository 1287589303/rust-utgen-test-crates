// Answer 0

#[test]
fn test_split_to_zero_length_input() {
    let mut b1 = BytesMut::from("");
    b1.split_to(0);
}

#[test]
fn test_split_to_full_length_input() {
    let mut b1 = BytesMut::from("hello");
    b1.split_to(5);
}

#[test]
fn test_split_to_partial_length_input() {
    let mut b1 = BytesMut::from("hello");
    b1.split_to(3);
}

#[test]
fn test_split_to_exceeding_length() {
    let mut b1 = BytesMut::from("hello");
    b1.split_to(6);
}

#[test]
fn test_split_to_string_with_space() {
    let mut b1 = BytesMut::from("hello world");
    b1.split_to(6);
}

#[test]
fn test_split_to_empty_input() {
    let mut b1 = BytesMut::from("");
    b1.split_to(0);
}

#[test]
fn test_split_to_one_character_input() {
    let mut b1 = BytesMut::from("a");
    b1.split_to(1);
}

#[test]
fn test_split_to_two_characters_input() {
    let mut b1 = BytesMut::from("ab");
    b1.split_to(2);
}

#[test]
fn test_split_to_three_characters_input() {
    let mut b1 = BytesMut::from("abc");
    b1.split_to(3);
}

#[test]
fn test_split_to_four_characters_input() {
    let mut b1 = BytesMut::from("abcd");
    b1.split_to(4);
}

#[test]
fn test_split_to_five_characters_input() {
    let mut b1 = BytesMut::from("abcde");
    b1.split_to(5);
}

#[test]
fn test_split_to_six_characters_input() {
    let mut b1 = BytesMut::from("abcdef");
    b1.split_to(6);
}

