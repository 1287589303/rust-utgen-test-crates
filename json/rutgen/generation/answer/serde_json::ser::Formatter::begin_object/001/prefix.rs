// Answer 0

#[test]
fn test_begin_object_with_vec_writer() {
    struct VecWriter(Vec<u8>);
    impl io::Write for VecWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = VecWriter(Vec::new());
    writer.begin_object().unwrap();
}

#[test]
fn test_begin_object_with_string_writer() {
    struct StringWriter(String);
    impl io::Write for StringWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.push_str(&String::from_utf8_lossy(buf));
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = StringWriter(String::new());
    writer.begin_object().unwrap();
}

#[test]
fn test_begin_object_with_empty_vec_writer() {
    struct EmptyVecWriter(Vec<u8>);
    impl io::Write for EmptyVecWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = EmptyVecWriter(Vec::new());
    writer.begin_object().unwrap();
}

#[test]
fn test_begin_object_with_large_vec_writer() {
    struct LargeVecWriter(Vec<u8>);
    impl io::Write for LargeVecWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = LargeVecWriter(vec![0; 1024]);
    writer.begin_object().unwrap();
}

