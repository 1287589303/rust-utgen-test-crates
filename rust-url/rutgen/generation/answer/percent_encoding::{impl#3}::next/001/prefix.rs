// Answer 0

#[test]
fn test_empty_slice() {
    let bytes: Vec<u8> = Vec::new();
    let mut decoder = PercentDecode { bytes: bytes.iter() };
    decoder.next();
}

#[test]
fn test_slice_no_percent_signs() {
    let bytes: Vec<u8> = vec![b'a', b'b', b'c'];
    let mut decoder = PercentDecode { bytes: bytes.iter() };
    decoder.next();
}

#[test]
fn test_slice_one_percent_sign_valid() {
    let bytes: Vec<u8> = vec![b'a', b'b', b'%', b'3', b'1'];
    let mut decoder = PercentDecode { bytes: bytes.iter() };
    decoder.next();
}

#[test]
fn test_slice_one_percent_sign_invalid() {
    let bytes: Vec<u8> = vec![b'a', b'b', b'%', b'g', b'h'];
    let mut decoder = PercentDecode { bytes: bytes.iter() };
    decoder.next();
}

#[test]
fn test_slice_multiple_percent_signs() {
    let bytes: Vec<u8> = vec![b'%', b'3', b'1', b'%', b'!', b'%', b'4', b'2'];
    let mut decoder = PercentDecode { bytes: bytes.iter() };
    decoder.next();
}

#[test]
fn test_slice_only_percent_signs() {
    let bytes: Vec<u8> = vec![b'%', b'%', b'%'];
    let mut decoder = PercentDecode { bytes: bytes.iter() };
    decoder.next();
}

