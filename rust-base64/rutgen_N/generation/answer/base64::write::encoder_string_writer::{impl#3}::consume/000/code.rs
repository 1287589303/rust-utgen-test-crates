// Answer 0

#[derive(Default)]
struct MockEncoder {
    consumed: String,
}

impl MockEncoder {
    fn consume(&mut self, buf: &str) {
        self.consumed.push_str(buf);
    }
}

#[test]
fn test_consume() {
    let mut encoder = MockEncoder::default();
    let test_str = "Hello, World!";
    
    encoder.consume(test_str);
    
    assert_eq!(encoder.consumed, test_str);
}

#[test]
fn test_consume_empty() {
    let mut encoder = MockEncoder::default();
    let test_str = "";
    
    encoder.consume(test_str);
    
    assert_eq!(encoder.consumed, test_str);
}

#[test]
fn test_consume_multiple_calls() {
    let mut encoder = MockEncoder::default();
    encoder.consume("First Call, ");
    encoder.consume("Second Call.");
    
    assert_eq!(encoder.consumed, "First Call, Second Call.");
}

