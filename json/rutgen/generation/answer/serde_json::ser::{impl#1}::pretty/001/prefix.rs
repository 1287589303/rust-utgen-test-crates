// Answer 0

#[test]
fn test_pretty_serializer_empty_writer() {
    struct EmptyWriter;
    
    impl io::Write for EmptyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = EmptyWriter;
    let serializer = Serializer::pretty(writer);
}

#[test]
fn test_pretty_serializer_small_writer() {
    struct SmallWriter(Vec<u8>);
    
    impl io::Write for SmallWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = SmallWriter(Vec::new());
    let serializer = Serializer::pretty(writer);
}

#[test]
fn test_pretty_serializer_large_writer() {
    struct LargeWriter(Vec<u8>);
    
    impl io::Write for LargeWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = LargeWriter(Vec::new());
    let serializer = Serializer::pretty(writer);
}

#[test]
fn test_pretty_serializer_non_ascii_writer() {
    struct NonAsciiWriter(Vec<u8>);
    
    impl io::Write for NonAsciiWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = NonAsciiWriter(Vec::new());
    let serializer = Serializer::pretty(writer);
}

