// Answer 0

#[test]
fn test_end_object_with_vec_writer() {
    struct TestFormatter;

    let mut buffer: Vec<u8> = Vec::new();
    let mut formatter = TestFormatter;
    formatter.end_object(&mut buffer).unwrap();
}

#[test]
fn test_end_object_with_string_writer() {
    struct TestFormatter;

    let mut buffer = String::new();
    let mut formatter = TestFormatter;
    formatter.end_object(&mut buffer).unwrap();
}

#[test]
fn test_end_object_with_file_writer() {
    struct TestFormatter;

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("test_output.txt").unwrap();
    let mut formatter = TestFormatter;
    formatter.end_object(&mut file).unwrap();
}

