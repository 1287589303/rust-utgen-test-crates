// Answer 0

#[test]
fn test_string_sink_new() {
    let mut sample_string = String::new();
    let sink = StringSink::new(&mut sample_string);
    assert_eq!(sink.string.len(), 0);
}

#[test]
fn test_string_sink_new_non_empty() {
    let mut sample_string = String::from("initial");
    let sink = StringSink::new(&mut sample_string);
    assert_eq!(sink.string.len(), 7);
}

