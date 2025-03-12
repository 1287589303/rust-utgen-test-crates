// Answer 0

#[test]
fn test_string_sink_new_valid() {
    let mut valid_string = String::from("test");
    let sink = StringSink::new(&mut valid_string);
}

#[test]
fn test_string_sink_new_empty() {
    let mut empty_string = String::new();
    let sink = StringSink::new(&mut empty_string);
}

#[test]
fn test_string_sink_new_large_string() {
    let mut large_string = String::from("a".repeat(1000));
    let sink = StringSink::new(&mut large_string);
}

#[test]
#[should_panic]
fn test_string_sink_new_invalid_string() {
    let string_ptr: *mut String = std::ptr::null_mut();
    let _sink = StringSink::new(unsafe { &mut *string_ptr });
}

