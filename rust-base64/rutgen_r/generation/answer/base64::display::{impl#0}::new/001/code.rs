// Answer 0

#[test]
fn test_base64_display_new() {
    struct DummyEngine;

    struct ChunkedEncoder<'e> {
        engine: &'e DummyEngine,
    }

    impl<'e> ChunkedEncoder<'e> {
        fn new(engine: &'e DummyEngine) -> Self {
            ChunkedEncoder { engine }
        }
    }

    struct Base64Display<'a, 'e, E> {
        bytes: &'a [u8],
        chunked_encoder: ChunkedEncoder<'e>,
    }

    fn new<'a, 'e, E>(bytes: &'a [u8], engine: &'e E) -> Base64Display<'a, 'e, E> {
        Base64Display {
            bytes,
            chunked_encoder: ChunkedEncoder::new(engine),
        }
    }

    let bytes: &[u8] = b"example";
    let engine = DummyEngine;

    let display = new(bytes, &engine);

    assert_eq!(display.bytes, bytes);
    assert_eq!(display.chunked_encoder.engine, &engine);
}

