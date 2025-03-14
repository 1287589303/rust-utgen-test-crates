// Answer 0

#[test]
fn test_write_empty_buffer() {
    struct MockConsumer;
    
    impl StrConsumer for MockConsumer {
        fn consume(&mut self, _buf: &str) {}
    }

    let mut consumer = MockConsumer;
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let result = writer.write(&[]);
}

#[test]
fn test_write_single_byte() {
    struct MockConsumer {
        consumed: String,
    }
    
    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = MockConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let byte: &[u8] = &[0x61]; // 'a'
    let result = writer.write(byte);
}

#[test]
fn test_write_multiple_bytes() {
    struct MockConsumer {
        consumed: String,
    }
    
    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = MockConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let bytes: &[u8] = &[0x61, 0x62, 0x63]; // 'abc'
    let result = writer.write(bytes);
}

#[test]
fn test_write_multi_byte_utf8() {
    struct MockConsumer {
        consumed: String,
    }
    
    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = MockConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let bytes: &[u8] = &[0xE2, 0x9C, 0x94]; // '✓' (check mark)
    let result = writer.write(bytes);
}

#[test]
fn test_write_large_valid_utf8() {
    struct MockConsumer {
        consumed: String,
    }
    
    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let mut consumer = MockConsumer { consumed: String::new() };
    let mut writer = Utf8SingleCodeUnitWriter { str_consumer: consumer };
    let bytes: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect(); // Practically valid UTF-8 range
    let result = writer.write(&bytes);
}

