// Answer 0

#[test]
fn test_write_null_to_byte_vector() {
    struct TestFormatter;

    let mut buf: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_null(&mut buf);
}

#[test]
fn test_write_null_to_string_writer() {
    struct TestFormatter;

    let mut buf = String::new();
    let mut formatter = TestFormatter;

    let _ = formatter.write_null(&mut buf);
}

#[test]
fn test_write_null_to_file_writer() {
    use std::fs::File;
    use std::io::{self, Write};

    struct TestFormatter;

    let temp_file = "test_write_null.tmp";
    let mut file = File::create(temp_file).unwrap();
    let mut formatter = TestFormatter;

    let _ = formatter.write_null(&mut file);
    let _ = std::fs::remove_file(temp_file);
}

#[test]
#[should_panic]
fn test_write_null_with_null_writer() {
    struct TestFormatter;

    let mut formatter = TestFormatter;

    let _ = formatter.write_null::<&mut dyn io::Write>(std::ptr::null_mut()).unwrap();
}

