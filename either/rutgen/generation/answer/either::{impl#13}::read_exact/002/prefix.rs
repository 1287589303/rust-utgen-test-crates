// Answer 0

#[test]
fn test_read_exact_left_with_empty_buffer() {
    struct MockReader;

    impl Read for MockReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Ok(0) // Simulate no bytes to read
        }
    }

    let mut buffer = [];
    let mut either = Either::Left(MockReader);
    let _result = either.read_exact(&mut buffer);
}

#[test]
fn test_read_exact_left_with_small_buffer() {
    struct MockReader {
        data: Vec<u8>,
        read_position: usize,
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let amount_to_read = (self.data.len() - self.read_position).min(buf.len());
            buf[..amount_to_read].copy_from_slice(&self.data[self.read_position..self.read_position + amount_to_read]);
            self.read_position += amount_to_read;
            Ok(amount_to_read)
        }
    }

    let mut buffer = [0; 5];
    let mut either = Either::Left(MockReader { data: vec![1, 2, 3, 4, 5], read_position: 0 });
    let _result = either.read_exact(&mut buffer);
}

#[test]
fn test_read_exact_left_with_exact_buffer_size() {
    struct MockReader {
        data: Vec<u8>,
        read_position: usize,
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let amount_to_read = (self.data.len() - self.read_position).min(buf.len());
            buf[..amount_to_read].copy_from_slice(&self.data[self.read_position..self.read_position + amount_to_read]);
            self.read_position += amount_to_read;
            Ok(amount_to_read)
        }
    }

    let mut buffer = [0; 3];
    let mut either = Either::Left(MockReader { data: vec![1, 2, 3], read_position: 0 });
    let _result = either.read_exact(&mut buffer);
}

#[test]
fn test_read_exact_right_with_non_zero_full_read() {
    struct MockReader {
        data: Vec<u8>,
        read_position: usize,
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let amount_to_read = (self.data.len() - self.read_position).min(buf.len());
            let bytes_read = if amount_to_read > 0 {
                buf[..amount_to_read].copy_from_slice(&self.data[self.read_position..self.read_position + amount_to_read]);
                self.read_position += amount_to_read;
            }
            Ok(amount_to_read)
        }
    }

    let mut buffer = [0; 3];
    let mut either = Either::Right(MockReader { data: vec![4, 5, 6], read_position: 0 });
    let _result = either.read_exact(&mut buffer);
}

