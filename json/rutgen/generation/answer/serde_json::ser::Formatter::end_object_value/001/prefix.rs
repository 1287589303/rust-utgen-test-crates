// Answer 0

#[test]
fn test_end_object_value_with_empty_writer() {
    struct EmptyWriter;
    
    impl io::Write for EmptyWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Ok(0)
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = EmptyWriter;
    let mut formatter = ();
    formatter.end_object_value(&mut writer).unwrap();
}

#[test]
fn test_end_object_value_with_buffer_writer() {
    let mut buffer = Vec::new();
    
    struct VecWriter<'a>(&'a mut Vec<u8>);
    
    impl io::Write for VecWriter<'_> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = VecWriter(&mut buffer);
    let mut formatter = ();
    formatter.end_object_value(&mut writer).unwrap();
}

#[test]
fn test_end_object_value_with_file_writer() {
    use std::fs::File;
    use std::io::Write;

    let file = File::create("test_output.txt").unwrap();
    let mut writer = file;
    let mut formatter = ();
    formatter.end_object_value(&mut writer).unwrap();
}

