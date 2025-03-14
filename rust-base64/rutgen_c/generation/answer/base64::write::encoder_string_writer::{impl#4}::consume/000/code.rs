// Answer 0

#[test]
fn test_str_consumer_consume_empty_string() {
    let mut consumer = String::new();
    consumer.consume("");
    assert_eq!(consumer, "");
}

#[test]
fn test_str_consumer_consume_non_empty_string() {
    let mut consumer = String::new();
    consumer.consume("Hello, ");
    consumer.consume("world!");
    assert_eq!(consumer, "Hello, world!");
}

#[test]
fn test_str_consumer_consume_multiple_calls() {
    let mut consumer = String::new();
    consumer.consume("First ");
    consumer.consume("Second ");
    consumer.consume("Third");
    assert_eq!(consumer, "First Second Third");
}

#[test]
fn test_str_consumer_consume_special_characters() {
    let mut consumer = String::new();
    consumer.consume("!@#$%^&*()_+");
    assert_eq!(consumer, "!@#$%^&*()_+");
}

#[test]
fn test_str_consumer_consume_large_string() {
    let mut consumer = String::new();
    let large_string = "A".repeat(1000);
    consumer.consume(&large_string);
    assert_eq!(consumer, large_string);
}

