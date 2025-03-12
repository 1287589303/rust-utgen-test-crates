// Answer 0

#[test]
fn test_read_exact_right_non_empty_buffer() {
    struct MockRead;
    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let data = b"hello";
            let len = data.len().min(buf.len());
            buf[..len].copy_from_slice(&data[..len]);
            Ok(len)
        }
    }

    let mut buf = [0u8; 5];
    let mut either = Either::Right(MockRead);
    either.read_exact(&mut buf).unwrap();
}

#[test]
fn test_read_exact_right_boundary_buffer_min() {
    struct MockRead;
    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf[0] = b'h';
            Ok(1)
        }
    }

    let mut buf = [0u8; 1];
    let mut either = Either::Right(MockRead);
    either.read_exact(&mut buf).unwrap();
}

#[test]
fn test_read_exact_right_boundary_buffer_max() {
    struct MockRead;
    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            for i in 0..buf.len() {
                buf[i] = i as u8;
            }
            Ok(buf.len())
        }
    }

    let mut buf = [0u8; 1024];
    let mut either = Either::Right(MockRead);
    either.read_exact(&mut buf).unwrap();
}

#[test]
fn test_read_exact_right_zero_length_buffer() {
    struct MockRead;
    impl Read for MockRead {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            // No data to read for zero-length buffer
            Ok(0)
        }
    }

    let mut buf: &mut [u8] = &mut [];
    let mut either = Either::Right(MockRead);
    either.read_exact(&mut buf).unwrap();
}

