// Answer 0

#[test]
fn test_either_left_write_non_empty() {
    struct Writer;
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
    }

    let mut either = Either::Left(Writer);
    let buf = b"Hello, World!";
    let _ = either.write(buf);
}

#[test]
fn test_either_left_write_empty() {
    struct Writer;
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
    }

    let mut either = Either::Left(Writer);
    let buf: &[u8] = b"";
    let _ = either.write(buf);
}

#[test]
fn test_either_right_write_non_empty() {
    struct Writer;
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
    }

    let mut either = Either::Right(Writer);
    let buf = b"Testing Right Write";
    let _ = either.write(buf);
}

#[test]
fn test_either_right_write_empty() {
    struct Writer;
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
    }

    let mut either = Either::Right(Writer);
    let buf: &[u8] = b"";
    let _ = either.write(buf);
}

#[test]
fn test_either_left_write_max_size() {
    struct Writer;
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
    }

    let mut either = Either::Left(Writer);
    let buf = vec![0u8; 64 * 1024]; // 64 KB
    let _ = either.write(&buf);
}

#[test]
fn test_either_right_write_max_size() {
    struct Writer;
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn write_all(&mut self, _buf: &[u8]) -> io::Result<()> {
            Ok(())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _fmt: fmt::Arguments<'_>) -> io::Result<()> {
            Ok(())
        }
    }

    let mut either = Either::Right(Writer);
    let buf = vec![0u8; 64 * 1024]; // 64 KB
    let _ = either.write(&buf);
}

