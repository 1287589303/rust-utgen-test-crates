// Answer 0

#[test]
fn test_write_all_left_empty_buffer() {
    struct MockWriter;

    impl Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> { Ok(0) }
        fn write_all(&mut self, _: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let mut left_writer = Either::Left(MockWriter);
    let buf: &[u8] = &[];

    let _ = left_writer.write_all(buf);
}

#[test]
fn test_write_all_left_non_empty_buffer() {
    struct MockWriter;

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let mut left_writer = Either::Left(MockWriter);
    let buf: &[u8] = &[1, 2, 3, 4, 5];

    let _ = left_writer.write_all(buf);
}

#[test]
fn test_write_all_right_empty_buffer() {
    struct MockWriter;

    impl Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> { Ok(0) }
        fn write_all(&mut self, _: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let mut right_writer = Either::Right(MockWriter);
    let buf: &[u8] = &[];

    let _ = right_writer.write_all(buf);
}

#[test]
fn test_write_all_right_non_empty_buffer() {
    struct MockWriter;

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> { Ok(buf.len()) }
        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> { Ok(()) }
        fn write_fmt(&mut self, _: fmt::Arguments<'_>) -> io::Result<()> { Ok(()) }
        fn flush(&mut self) -> io::Result<()> { Ok(()) }
    }

    let mut right_writer = Either::Right(MockWriter);
    let buf: &[u8] = &[1, 2, 3, 4, 5];

    let _ = right_writer.write_all(buf);
}

