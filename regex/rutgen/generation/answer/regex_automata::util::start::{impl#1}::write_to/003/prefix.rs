// Answer 0

#[test]
fn test_write_to_success() {
    struct TestLookMatcher;

    let lookm = TestLookMatcher;
    let start_byte_map = StartByteMap::new(&lookm);
    let mut buffer = [0u8; 256];

    let result = start_byte_map.write_to(&mut buffer);
}

#[test]
fn test_write_to_buffer_too_small() {
    struct TestLookMatcher;

    let lookm = TestLookMatcher;
    let start_byte_map = StartByteMap::new(&lookm);
    let mut buffer = [0u8; 255];

    let result = start_byte_map.write_to(&mut buffer);
}

