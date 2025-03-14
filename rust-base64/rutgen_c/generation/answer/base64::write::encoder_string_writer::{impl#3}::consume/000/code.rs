// Answer 0

#[test]
fn test_consume_with_empty_string() {
    struct TestStrConsumer {
        data: String,
    }
    
    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut consumer = TestStrConsumer { data: String::new() };
    consumer.consume("Hello");
    assert_eq!(consumer.data, "Hello");
}

#[test]
fn test_consume_with_non_empty_string() {
    struct TestStrConsumer {
        data: String,
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut consumer = TestStrConsumer { data: String::from("World") };
    consumer.consume("!");
    assert_eq!(consumer.data, "World!");
}

#[test]
fn test_consume_multiple_times() {
    struct TestStrConsumer {
        data: String,
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut consumer = TestStrConsumer { data: String::new() };
    consumer.consume("This ");
    consumer.consume("is a ");
    consumer.consume("test.");
    assert_eq!(consumer.data, "This is a test.");
}

#[test]
fn test_consume_empty_buffer() {
    struct TestStrConsumer {
        data: String,
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.data.push_str(buf);
        }
    }

    let mut consumer = TestStrConsumer { data: String::from("Initial ") };
    consumer.consume("");
    assert_eq!(consumer.data, "Initial ");
}

