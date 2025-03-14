// Answer 0

#[test]
fn test_string_sink_new() {
    let mut test_string = String::new();
    let sink = StringSink::new(&mut test_string);
    assert_eq!(sink.string, &test_string);
} 

#[test]
fn test_string_sink_new_with_initial_value() {
    let mut test_string = String::from("initial value");
    let sink = StringSink::new(&mut test_string);
    assert_eq!(sink.string, &test_string);
} 

#[test]
fn test_string_sink_new_empty_string() {
    let mut test_string = String::new();
    let sink = StringSink::new(&mut test_string);
    assert!(sink.string.is_empty());
} 

#[test]
fn test_string_sink_new_with_capacity() {
    let mut test_string = String::with_capacity(10);
    let sink = StringSink::new(&mut test_string);
    assert_eq!(sink.string.capacity(), 10);
}

