// Answer 0

#[test]
fn test_write_to_succeeds_with_exact_size_buffer() {
    let lookm = LookMatcher::new();
    let start_byte_map = StartByteMap::new(&lookm);
    let mut buffer = [0u8; 256];
    let result = start_byte_map.write_to(&mut buffer);
}

#[test]
fn test_write_to_fails_with_too_small_buffer() {
    let lookm = LookMatcher::new();
    let start_byte_map = StartByteMap::new(&lookm);
    let mut buffer = [0u8; 255];
    let result = start_byte_map.write_to(&mut buffer);
}

#[test]
fn test_write_to_fails_with_zero_length_buffer() {
    let lookm = LookMatcher::new();
    let start_byte_map = StartByteMap::new(&lookm);
    let mut buffer = [];
    let result = start_byte_map.write_to(&mut buffer);
}

#[test]
fn test_write_to_fails_with_exceeding_buffer() {
    let lookm = LookMatcher::new();
    let start_byte_map = StartByteMap::new(&lookm);
    let mut buffer = [0u8; 300];
    let result = start_byte_map.write_to(&mut buffer);
}

