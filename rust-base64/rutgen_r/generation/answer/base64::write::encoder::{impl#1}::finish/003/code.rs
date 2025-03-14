// Answer 0

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_with_none_delegate() {
    struct Encoder<W> {
        delegate: Option<W>,
    }

    impl<W> Encoder<W> {
        pub fn new() -> Self {
            Encoder { delegate: None }
        }

        pub fn finish(&mut self) -> Result<W, &'static str> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            // Simulate write_final_leftovers
            // For this test case, we do not need to implement this method.
            // Just trying to keep the signature intact.

            let writer = self.delegate.take().expect("Writer must be present");
            Ok(writer)
        }
    }

    let mut encoder: Encoder<()> = Encoder::new();
    let _ = encoder.finish(); // This should panic
}

#[test]
fn test_finish_with_some_delegate() {
    struct TestWriter;
    struct Encoder<W> {
        delegate: Option<W>,
    }

    impl<W> Encoder<W> {
        pub fn new(writer: W) -> Self {
            Encoder { delegate: Some(writer) }
        }

        pub fn finish(&mut self) -> Result<W, &'static str> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            // Simulating write_final_leftovers successful execution
            // In a real scenario implement the actual logic

            let writer = self.delegate.take().expect("Writer must be present");
            Ok(writer)
        }
    }

    let writer = TestWriter;
    let mut encoder: Encoder<TestWriter> = Encoder::new(writer);
    let _ = encoder.finish().unwrap(); // This should succeed
}

