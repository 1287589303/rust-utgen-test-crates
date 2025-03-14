// Answer 0

#[test]
fn test_consume_into_string() {
    let mut string = String::new();
    string.consume("Hello");
    assert_eq!(string, "Hello");
}

#[test]
fn test_consume_multiple_times() {
    let mut string = String::new();
    string.consume("Hello");
    string.consume(", ");
    string.consume("World!");
    assert_eq!(string, "Hello, World!");
}

#[test]
fn test_consume_empty_string() {
    let mut string = String::new();
    string.consume("");
    assert_eq!(string, "");
}

#[test]
fn test_consume_with_initial_value() {
    let mut string = String::from("Initial");
    string.consume(" value");
    assert_eq!(string, "Initial value");
}

