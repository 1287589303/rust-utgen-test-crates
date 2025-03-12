// Answer 0

#[test]
fn test_write_to_len() {
    let start_byte_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let len = start_byte_map.write_to_len();
}

#[test]
fn test_write_to_len_boundary() {
    let start_byte_map = StartByteMap { map: [Start::WordByte; 256] };
    let len = start_byte_map.write_to_len();
}

#[test]
fn test_write_to_len_empty_case() {
    let start_byte_map = StartByteMap { map: [Start::Text; 256] };
    let len = start_byte_map.write_to_len();
}

