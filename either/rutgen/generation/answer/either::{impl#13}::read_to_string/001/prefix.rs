// Answer 0

#[test]
fn test_read_to_string_with_non_empty_buffer() {
    struct MockReader {
        content: String,
        position: usize,
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let bytes_to_read = buf.len().min(self.content.len() - self.position);
            let bytes = self.content[self.position..self.position + bytes_to_read].as_bytes();
            buf[..bytes_to_read].copy_from_slice(bytes);
            self.position += bytes_to_read;
            Ok(bytes_to_read)
        }
        
        fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
            let bytes_read = self.read(buf.as_mut_bytes())?;
            buf.truncate(bytes_read);
            Ok(bytes_read)
        }
    }

    let mock_reader = MockReader {
        content: "Hello, world!".to_string(),
        position: 0,
    };
    
    let mut buf = String::new();
    let either: Either<MockReader, MockReader> = Right(mock_reader);
    
    let _ = either.read_to_string(&mut buf);
}

#[test]
fn test_read_to_string_with_empty_buffer() {
    struct MockReader {
        content: String,
        position: usize,
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            Ok(0)
        }
        
        fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
            Ok(0)
        }
    }

    let mock_reader = MockReader {
        content: "".to_string(),
        position: 0,
    };
    
    let mut buf = String::new();
    let either: Either<MockReader, MockReader> = Right(mock_reader);
    
    let _ = either.read_to_string(&mut buf);
}

