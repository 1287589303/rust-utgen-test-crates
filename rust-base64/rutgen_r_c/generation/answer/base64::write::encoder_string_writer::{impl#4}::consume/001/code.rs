// Answer 0

#[test]
fn test_consume_empty_string() {
    let mut consumer = String::new();
    consumer.consume("");
    assert_eq!(consumer, "");
}

#[test]
fn test_consume_non_empty_string() {
    let mut consumer = String::new();
    consumer.consume("Hello, ");
    assert_eq!(consumer, "Hello, ");
}

#[test]
fn test_consume_multiple_calls() {
    let mut consumer = String::new();
    consumer.consume("Hello");
    consumer.consume(", World");
    assert_eq!(consumer, "Hello, World");
}

#[test]
fn test_consume_long_string() {
    let mut consumer = String::new();
    let long_str = "A long string that exceeds typical lengths for testing purposes.";
    consumer.consume(long_str);
    assert_eq!(consumer, long_str);
}

#[test]
fn test_consume_special_characters() {
    let mut consumer = String::new();
    consumer.consume("Special characters: !@#$%^&*()");
    assert_eq!(consumer, "Special characters: !@#$%^&*()");
}

#[test]
fn test_consume_concurrent_strings() {
    let mut consumer = String::new();
    consumer.consume("First part. ");
    consumer.consume("Second part.");
    assert_eq!(consumer, "First part. Second part.");
}

