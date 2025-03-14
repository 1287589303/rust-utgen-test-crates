// Answer 0

#[test]
fn test_into_inner_returns_inner_reader() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.position >= self.data.len() {
                return Ok(0);
            }
            let bytes_to_read = std::cmp::min(buf.len(), self.data.len() - self.position);
            buf[..bytes_to_read].copy_from_slice(&self.data[self.position..self.position + bytes_to_read]);
            self.position += bytes_to_read;
            Ok(bytes_to_read)
        }
    }

    struct DecoderReader<R> {
        inner: R,
    }

    impl<R: std::io::Read> DecoderReader<R> {
        pub fn new(inner: R) -> Self {
            DecoderReader { inner }
        }

        pub fn into_inner(self) -> R {
            self.inner
        }
    }

    let mock_data = vec![104, 101, 108, 108, 111]; // "hello"
    let mock_reader = MockReader {
        data: mock_data.clone(),
        position: 0,
    };

    let decoder_reader = DecoderReader::new(mock_reader);
    let inner_reader = decoder_reader.into_inner();

    let mut buffer = vec![0; 5];
    let bytes_read = inner_reader.read(&mut buffer).unwrap();

    assert_eq!(bytes_read, 5);
    assert_eq!(buffer, mock_data);
}

#[test]
fn test_into_inner_empty_reader() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl std::io::Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            Ok(0)
        }
    }

    struct DecoderReader<R> {
        inner: R,
    }

    impl<R: std::io::Read> DecoderReader<R> {
        pub fn new(inner: R) -> Self {
            DecoderReader { inner }
        }
        
        pub fn into_inner(self) -> R {
            self.inner
        }
    }

    let empty_reader = MockReader {
        data: vec![],
        position: 0,
    };

    let decoder_reader = DecoderReader::new(empty_reader);
    let inner_reader = decoder_reader.into_inner();

    let mut buffer = vec![0; 5];
    let bytes_read = inner_reader.read(&mut buffer).unwrap();

    assert_eq!(bytes_read, 0);
}

