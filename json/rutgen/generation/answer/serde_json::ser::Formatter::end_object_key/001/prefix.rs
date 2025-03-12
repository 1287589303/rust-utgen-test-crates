// Answer 0

#[test]
fn test_end_object_key_for_vec_u8_writer() {
    struct TestFormatter;

    let mut writer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.end_object_key(&mut writer);
}

#[test]
fn test_end_object_key_for_string_writer() {
    struct TestFormatter;

    let mut writer: String = String::new();
    let mut formatter = TestFormatter;

    let _ = formatter.end_object_key(&mut writer);
}

#[test]
fn test_end_object_key_for_slice_writer() {
    struct TestFormatter;

    let mut writer: &mut [u8] = &mut [0; 10];
    let mut formatter = TestFormatter;

    let _ = formatter.end_object_key(&mut writer);
}

#[test]
fn test_end_object_key_for_box_writer() {
    struct TestFormatter;

    let mut writer: Box<Vec<u8>> = Box::new(Vec::new());
    let mut formatter = TestFormatter;

    let _ = formatter.end_object_key(&mut writer);
}

#[test]
fn test_end_object_key_for_custom_writer() {
    struct CustomWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for CustomWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct TestFormatter;

    let mut writer = CustomWriter { buffer: Vec::new() };
    let mut formatter = TestFormatter;

    let _ = formatter.end_object_key(&mut writer);
}

