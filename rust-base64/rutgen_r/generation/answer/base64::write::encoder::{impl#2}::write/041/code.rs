// Answer 0

#[test]
#[should_panic(expected = "Cannot write more after calling finish()")]
fn test_write_with_none_delegate() {
    struct TestWriter {
        delegate: Option<()>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                delegate: None,
            }
        }

        fn write(&mut self, input: &[u8]) -> Result<usize, &'static str> {
            assert!(
                self.delegate.is_some(),
                "Cannot write more after calling finish()"
            );
            Ok(input.len())
        }
    }

    let mut writer = TestWriter::new();
    let _ = writer.write(&[1, 2, 3]);
}

