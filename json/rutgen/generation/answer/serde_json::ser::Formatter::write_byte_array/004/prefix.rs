// Answer 0

#[test]
fn test_write_byte_array_empty() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let byte_array: Vec<u8> = vec![];
    let _ = formatter.write_byte_array(&mut writer, &byte_array);
}

#[test]
fn test_write_byte_array_one_byte() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let byte_array: Vec<u8> = vec![42];
    let _ = formatter.write_byte_array(&mut writer, &byte_array);
}

#[test]
fn test_write_byte_array_multiple_bytes() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let byte_array: Vec<u8> = vec![1, 2, 3, 4, 5];
    let _ = formatter.write_byte_array(&mut writer, &byte_array);
}

#[test]
#[should_panic]
fn test_write_byte_array_exceeding_capacity() {
    struct TestFormatter;

    impl Formatter for TestFormatter {}

    let mut writer = Vec::new();
    let formatter = TestFormatter;
    let byte_array: Vec<u8> = (0..1025).map(|x| x as u8).collect(); // Exceeding assumed limit
    let _ = formatter.write_byte_array(&mut writer, &byte_array);
}

