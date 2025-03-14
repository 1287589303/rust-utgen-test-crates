// Answer 0

#[test]
fn test_read_from_delegate_success() {
    use std::io::{self, Cursor};

    const BUF_SIZE: usize = 1024;

    struct TestReader {
        inner: Cursor<Vec<u8>>,
        b64_buffer: Vec<u8>,
        b64_offset: usize,
        b64_len: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>, b64_offset: usize, b64_len: usize) -> Self {
            Self {
                inner: Cursor::new(data),
                b64_buffer: vec![0; BUF_SIZE],
                b64_offset,
                b64_len,
            }
        }

        fn read_from_delegate(&mut self) -> io::Result<usize> {
            debug_assert!(self.b64_offset + self.b64_len < BUF_SIZE);
            let read = self
                .inner
                .read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
            self.b64_len += read;
            debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
            Ok(read)
        }
    }

    let mut reader = TestReader::new(vec![1, 2, 3], 0, 0);
    let result = reader.read_from_delegate();
    assert_eq!(result.unwrap(), 3);
    assert_eq!(reader.b64_len, 3);
}

#[test]
#[should_panic]
fn test_read_from_delegate_error() {
    use std::io::{self, Cursor};

    const BUF_SIZE: usize = 1024;

    struct TestReader {
        inner: Cursor<Vec<u8>>,
        b64_buffer: Vec<u8>,
        b64_offset: usize,
        b64_len: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>, b64_offset: usize, b64_len: usize) -> Self {
            Self {
                inner: Cursor::new(data),
                b64_buffer: vec![0; BUF_SIZE],
                b64_offset,
                b64_len,
            }
        }

        fn read_from_delegate(&mut self) -> io::Result<usize> {
            debug_assert!(self.b64_offset + self.b64_len < BUF_SIZE);
            let read = self
                .inner
                .read(&mut self.b64_buffer[self.b64_offset + self.b64_len..])?;
            self.b64_len += read;
            debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
            Ok(read)
        }
    }

    let mut reader = TestReader::new(vec![], 0, 0);
    let _ = reader.read_from_delegate().unwrap();
}

