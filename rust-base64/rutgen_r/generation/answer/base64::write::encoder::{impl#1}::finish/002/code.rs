// Answer 0

#[test]
fn test_finish_success() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: Vec::new() }
        }

        fn output(&self) -> &[u8] {
            &self.output
        }
    }

    use std::io::{Result, Write};

    struct Encoder<W: Write> {
        delegate: Option<W>,
    }

    impl<W: Write> Encoder<W> {
        fn new(writer: W) -> Self {
            Encoder {
                delegate: Some(writer),
            }
        }

        fn write_final_leftovers(&mut self) -> Result<()> {
            // Simulate finalization process that succeeds
            Ok(())
        }

        pub fn finish(&mut self) -> Result<W> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            self.write_final_leftovers()?;

            let writer = self.delegate.take().expect("Writer must be present");

            Ok(writer)
        }
    }

    let mut writer = DummyWriter::new();
    let mut encoder = Encoder::new(&mut writer);
    let result = encoder.finish();
    assert!(result.is_ok());
}

#[test]
fn test_finish_already_finished() {
    struct DummyWriter {
        output: Vec<u8>,
    }

    impl DummyWriter {
        fn new() -> Self {
            DummyWriter { output: Vec::new() }
        }
    }

    use std::io::{Result, Write};

    struct Encoder<W: Write> {
        delegate: Option<W>,
    }

    impl<W: Write> Encoder<W> {
        fn new(writer: W) -> Self {
            Encoder {
                delegate: Some(writer),
            }
        }

        fn write_final_leftovers(&mut self) -> Result<()> {
            // Simulate finalization process that succeeds
            Ok(())
        }

        pub fn finish(&mut self) -> Result<W> {
            assert!(
                self.delegate.is_some(),
                "Encoder has already had finish() called"
            );

            self.write_final_leftovers()?;

            let writer = self.delegate.take().expect("Writer must be present");

            Ok(writer)
        }
    }

    let mut writer = DummyWriter::new();
    let mut encoder = Encoder::new(&mut writer);
    let _ = encoder.finish();

    let result = std::panic::catch_unwind(|| {
        encoder.finish().unwrap();
    });
    assert!(result.is_err());
}

