// Answer 0

#[test]
fn test_into_inner_with_empty_input() {
    struct Encoder {
        finished: bool,
        str_consumer: String,
    }

    impl Encoder {
        fn finish(self) -> Result<Self, &'static str> {
            if self.finished {
                Ok(self)
            } else {
                Err("Not finished")
            }
        }
    }

    struct StringWriter {
        encoder: Encoder,
    }

    impl StringWriter {
        fn new() -> StringWriter {
            StringWriter {
                encoder: Encoder {
                    finished: true,
                    str_consumer: String::new(),
                },
            }
        }

        fn into_inner(mut self) -> Encoder {
            self.encoder
                .finish()
                .expect("Writing to a consumer should never fail")
        }
    }

    let writer = StringWriter::new();
    let encoder = writer.into_inner();
    assert_eq!(encoder.str_consumer, "");
}

#[test]
fn test_into_inner_with_data() {
    struct Encoder {
        finished: bool,
        str_consumer: String,
    }

    impl Encoder {
        fn finish(self) -> Result<Self, &'static str> {
            if self.finished {
                Ok(self)
            } else {
                Err("Not finished")
            }
        }
    }

    struct StringWriter {
        encoder: Encoder,
    }

    impl StringWriter {
        fn new(data: String) -> StringWriter {
            StringWriter {
                encoder: Encoder {
                    finished: true,
                    str_consumer: data,
                },
            }
        }

        fn into_inner(mut self) -> Encoder {
            self.encoder
                .finish()
                .expect("Writing to a consumer should never fail")
        }
    }

    let writer = StringWriter::new("Hello, World!".to_string());
    let encoder = writer.into_inner();
    assert_eq!(encoder.str_consumer, "Hello, World!");
}

