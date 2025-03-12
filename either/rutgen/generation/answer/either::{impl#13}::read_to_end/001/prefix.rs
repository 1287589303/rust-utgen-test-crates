// Answer 0

#[test]
fn test_read_to_end_with_right_variant() {
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

        fn read_to_end(&mut self, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> {
            let len = self.data.len();
            buf.extend(self.data.clone());
            self.data.clear();
            Ok(len)
        }
    }

    let test_data = vec![1, 2, 3, 4, 5];
    let test_reader = TestReader { data: test_data };
    let either_instance = Right(test_reader);
    let mut buffer = Vec::with_capacity(10);

    let _ = either_instance.read_to_end(&mut buffer);
}

#[test]
fn test_read_to_end_with_empty_right_variant() {
    struct EmptyReader;

    impl Read for EmptyReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
    }

    let empty_reader = EmptyReader;
    let either_instance = Right(empty_reader);
    let mut buffer = Vec::with_capacity(10);

    let _ = either_instance.read_to_end(&mut buffer);
}

#[test]
fn test_read_to_end_with_large_buffer() {
    struct LargeDataReader {
        data: Vec<u8>,
    }

    impl Read for LargeDataReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let len = std::cmp::min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data.drain(..len);
            Ok(len)
        }
    }

    let large_data = vec![0u8; 1024]; // 1 KB of data
    let large_data_reader = LargeDataReader { data: large_data };
    let either_instance = Right(large_data_reader);
    let mut buffer = Vec::with_capacity(2048); // 2 KB buffer

    let _ = either_instance.read_to_end(&mut buffer);
}

