// Answer 0

#[test]
fn test_write_to_delegate_success() {
    struct FakeWriter {
        written: Vec<u8>,
    }

    impl FakeWriter {
        fn new() -> Self {
            Self { written: Vec::new() }
        }
    }

    impl std::io::Write for FakeWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        output: Vec<u8>,
        output_occupied_len: usize,
        delegate: Option<FakeWriter>,
        panicked: bool,
    }

    impl Encoder {
        fn new(delegate: FakeWriter) -> Self {
            Self {
                output: vec![1, 2, 3, 4, 5], // Mock output for testing
                output_occupied_len: 5,
                delegate: Some(delegate),
                panicked: false,
            }
        }

        fn write_to_delegate(&mut self, current_output_len: usize) -> std::io::Result<()> {
            self.panicked = true;
            let res = self
                .delegate
                .as_mut()
                .expect("Writer must be present")
                .write(&self.output[..current_output_len]);
            self.panicked = false;

            res.map(|consumed| {
                debug_assert!(consumed <= current_output_len);

                if consumed < current_output_len {
                    self.output_occupied_len = current_output_len.checked_sub(consumed).unwrap();
                    self.output.rotate_left(consumed);
                } else {
                    self.output_occupied_len = 0;
                }
            })
        }
    }

    let fake_writer = FakeWriter::new();
    let mut encoder = Encoder::new(fake_writer);
    let result = encoder.write_to_delegate(5);

    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 0);
}

#[test]
fn test_write_to_delegate_partial_success() {
    struct FakeWriter {
        written: Vec<u8>,
    }

    impl FakeWriter {
        fn new() -> Self {
            Self { written: Vec::new() }
        }
    }

    impl std::io::Write for FakeWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            // Simulate partial write
            let len_to_write = buf.len() / 2; // Write half of what is passed
            self.written.extend_from_slice(&buf[..len_to_write]);
            Ok(len_to_write)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        output: Vec<u8>,
        output_occupied_len: usize,
        delegate: Option<FakeWriter>,
        panicked: bool,
    }

    impl Encoder {
        fn new(delegate: FakeWriter) -> Self {
            Self {
                output: vec![1, 2, 3, 4, 5],
                output_occupied_len: 5,
                delegate: Some(delegate),
                panicked: false,
            }
        }

        fn write_to_delegate(&mut self, current_output_len: usize) -> std::io::Result<()> {
            self.panicked = true;
            let res = self
                .delegate
                .as_mut()
                .expect("Writer must be present")
                .write(&self.output[..current_output_len]);
            self.panicked = false;

            res.map(|consumed| {
                debug_assert!(consumed <= current_output_len);

                if consumed < current_output_len {
                    self.output_occupied_len = current_output_len.checked_sub(consumed).unwrap();
                    self.output.rotate_left(consumed);
                } else {
                    self.output_occupied_len = 0;
                }
            })
        }
    }

    let fake_writer = FakeWriter::new();
    let mut encoder = Encoder::new(fake_writer);
    let result = encoder.write_to_delegate(5);

    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 2);
    assert_eq!(encoder.output, vec![3, 4, 5, 1, 2]);
}

#[should_panic]
#[test]
fn test_write_to_delegate_panic_on_missing_writer() {
    struct Encoder {
        output: Vec<u8>,
        output_occupied_len: usize,
        delegate: Option<FakeWriter>,
        panicked: bool,
    }

    impl Encoder {
        fn new() -> Self {
            Self {
                output: vec![1, 2, 3, 4, 5],
                output_occupied_len: 5,
                delegate: None,
                panicked: false,
            }
        }

        fn write_to_delegate(&mut self, current_output_len: usize) -> std::io::Result<()> {
            self.panicked = true;
            let res = self
                .delegate
                .as_mut()
                .expect("Writer must be present")
                .write(&self.output[..current_output_len]);
            self.panicked = false;

            res.map(|consumed| {
                if consumed < current_output_len {
                    self.output_occupied_len = current_output_len.checked_sub(consumed).unwrap();
                    self.output.rotate_left(consumed);
                } else {
                    self.output_occupied_len = 0;
                }
            })
        }
    }

    let mut encoder = Encoder::new();
    encoder.write_to_delegate(5).unwrap();
}

