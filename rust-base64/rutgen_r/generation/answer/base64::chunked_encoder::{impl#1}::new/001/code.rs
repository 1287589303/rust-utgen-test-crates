// Answer 0

#[test]
fn test_new_creates_string_sink() {
    let mut test_string = String::from("test");
    let string_sink = new(&mut test_string);
    assert_eq!(string_sink.string, "test");
}

#[test]
fn test_new_creates_empty_string_sink() {
    let mut test_string = String::new();
    let string_sink = new(&mut test_string);
    assert_eq!(string_sink.string, "");
}

