// Answer 0

#[test]
fn test_byte_offset_initial() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    let offset = stream.byte_offset(); // should be 0
}

#[test]
fn test_byte_offset_after_first_success() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    let _ = stream.next(); // Read first element
    let offset = stream.byte_offset(); // should be 3
}

#[test]
fn test_byte_offset_after_second_success() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    let _ = stream.next(); // Read first element
    let _ = stream.next(); // Read second element
    let offset = stream.byte_offset(); // should be 7
}

#[test]
fn test_byte_offset_after_eof_error() {
    let data = b"[0] [1] [";
    let de = serde_json::Deserializer::from_slice(data);
    let mut stream = de.into_iter::<Vec<i32>>();
    let _ = stream.next(); // Read first element
    let _ = stream.next(); // Read second element
    let result = stream.next(); // This should result in an error
    let offset = stream.byte_offset(); // should be 8
}

