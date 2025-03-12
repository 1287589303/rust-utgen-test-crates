// Answer 0

#[test]
fn test_read_to_string_left_non_empty() {
    struct TestReader {
        data: Vec<u8>,
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data.drain(..len);
            Ok(len)
        }
    }

    let test_data = TestReader {
        data: b"Hello, world!".to_vec(),
    };
    let mut buf = String::new();
    let either_instance = Either::Left(test_data);
    let _ = either_instance.read_to_string(&mut buf);
}

#[test]
fn test_read_to_string_left_empty() {
    struct TestReader {
        data: Vec<u8>,
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data.drain(..len);
            Ok(len)
        }
    }

    let test_data = TestReader {
        data: b"".to_vec(),
    };
    let mut buf = String::new();
    let either_instance = Either::Left(test_data);
    let _ = either_instance.read_to_string(&mut buf);
}

#[test]
fn test_read_to_string_left_partial_read() {
    struct TestReader {
        data: Vec<u8>,
    }

    impl Read for TestReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data.drain(..len);
            Ok(len)
        }
    }
    
    let test_data = TestReader {
        data: b"Hello".to_vec(),
    };
    let mut buf = String::new();
    let either_instance = Either::Left(test_data);
    let _ = either_instance.read_to_string(&mut buf);
}

