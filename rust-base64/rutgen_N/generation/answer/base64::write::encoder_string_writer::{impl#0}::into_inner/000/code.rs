// Answer 0

#[test]
fn test_into_inner() {
    struct MockEncoder {
        finished: bool,
        str_consumer: String,
    }

    impl MockEncoder {
        fn finish(&mut self) -> Result<&mut Self, &'static str> {
            if self.finished {
                Ok(self)
            } else {
                self.finished = true;
                Ok(self)
            }
        }
    }

    struct EncoderStringWriter {
        encoder: MockEncoder,
    }

    impl EncoderStringWriter {
        pub fn new() -> Self {
            EncoderStringWriter {
                encoder: MockEncoder {
                    finished: false,
                    str_consumer: String::from("encoded_data"),
                },
            }
        }

        pub fn into_inner(mut self) -> String {
            self.encoder
                .finish()
                .expect("Writing to a consumer should never fail")
                .str_consumer
        }
    }

    let writer = EncoderStringWriter::new();
    let result = writer.into_inner();
    assert_eq!(result, "encoded_data");
}

