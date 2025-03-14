// Answer 0

#[test]
fn test_encode_engine_string() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
            let input_bytes = input.as_ref();
            let encoded = base64::encode(input_bytes);
            output_buf.push_str(&encoded);
        }
    }

    let mut output_buf = String::new();
    let engine = MockEngine;

    encode_engine_string(b"test input", &mut output_buf, &engine);
    assert_eq!(output_buf, "dGVzdCBpbnB1dA==");
}

#[test]
fn test_encode_engine_string_empty() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
            let input_bytes = input.as_ref();
            let encoded = base64::encode(input_bytes);
            output_buf.push_str(&encoded);
        }
    }

    let mut output_buf = String::new();
    let engine = MockEngine;

    encode_engine_string(b"", &mut output_buf, &engine);
    assert_eq!(output_buf, "");
}

#[test]
fn test_encode_engine_string_single_byte() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
            let input_bytes = input.as_ref();
            let encoded = base64::encode(input_bytes);
            output_buf.push_str(&encoded);
        }
    }

    let mut output_buf = String::new();
    let engine = MockEngine;

    encode_engine_string(b"A", &mut output_buf, &engine);
    assert_eq!(output_buf, "QQ==");
}

