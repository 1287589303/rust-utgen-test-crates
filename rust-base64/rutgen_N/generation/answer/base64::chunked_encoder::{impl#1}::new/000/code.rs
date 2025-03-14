// Answer 0

#[test]
fn test_new_string_sink() {
    let mut test_string = String::new();
    let string_sink = new(&mut test_string);
    
    assert_eq!(string_sink.string, &test_string);
}

