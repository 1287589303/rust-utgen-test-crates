// Answer 0

#[test]
fn test_split_off_empty_string() {
    let mut b1 = BytesMut::from("");
    let _ = b1.split_off(0);
}

#[test]
fn test_split_off_single_character() {
    let mut b1 = BytesMut::from("a");
    let _ = b1.split_off(0);
    let _ = b1.split_off(1);
}

#[test]
fn test_split_off_two_characters() {
    let mut b1 = BytesMut::from("ab");
    let _ = b1.split_off(0);
    let _ = b1.split_off(1);
    let _ = b1.split_off(2);
}

#[test]
fn test_split_off_ten_characters() {
    let mut b1 = BytesMut::from("abcdefghij");
    let _ = b1.split_off(0);
    let _ = b1.split_off(5);
    let _ = b1.split_off(10);
}

#[test]
fn test_split_off_eleven_characters() {
    let mut b1 = BytesMut::from("abcdefghijk");
    let _ = b1.split_off(0);
    let _ = b1.split_off(5);
    let _ = b1.split_off(11);
}

#[test]
fn test_split_off_full_range() {
    let mut b1 = BytesMut::from("hello world");
    let _ = b1.split_off(0);
    let _ = b1.split_off(6);
    let _ = b1.split_off(11);
}

