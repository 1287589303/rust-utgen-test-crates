// Answer 0

#[test]
fn test_either_left_read_empty_buffer() {
    struct LeftReader;
    impl Read for LeftReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            Ok(0) // Simulate reading 0 bytes
        }
    }

    let mut buf = [0u8; 0];
    let mut either = Either::Left(LeftReader);
    let _ = either.read(&mut buf);
}

#[test]
fn test_either_left_read_small_buffer() {
    struct LeftReader;
    impl Read for LeftReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            buf[0] = 1; // Simulate reading 1 byte
            Ok(1)
        }
    }

    let mut buf = [0u8; 1];
    let mut either = Either::Left(LeftReader);
    let _ = either.read(&mut buf);
}

#[test]
fn test_either_left_read_full_buffer() {
    struct LeftReader;
    impl Read for LeftReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            for i in 0..buf.len() {
                buf[i] = (i + 1) as u8; // Simulate reading full buffer
            }
            Ok(buf.len())
        }
    }

    let mut buf = [0u8; 4096];
    let mut either = Either::Left(LeftReader);
    let _ = either.read(&mut buf);
}

#[test]
fn test_either_left_read_partial_buffer() {
    struct LeftReader;
    impl Read for LeftReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            for i in 0..(buf.len() / 2) {
                buf[i] = (i + 1) as u8; // Simulate reading half of the buffer
            }
            Ok(buf.len() / 2)
        }
    }

    let mut buf = [0u8; 1024];
    let mut either = Either::Left(LeftReader);
    let _ = either.read(&mut buf);
}

#[test]
fn test_either_left_read_max_buffer() {
    struct LeftReader;
    impl Read for LeftReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            for i in 0..buf.len() {
                buf[i] = (i % 256) as u8; // Simulate filling the buffer
            }
            Ok(buf.len())
        }
    }

    let mut buf = [0u8; 4096];
    let mut either = Either::Left(LeftReader);
    let _ = either.read(&mut buf);
}

