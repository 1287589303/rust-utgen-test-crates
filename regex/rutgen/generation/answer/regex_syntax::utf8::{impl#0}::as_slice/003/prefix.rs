// Answer 0

#[test]
fn test_as_slice_two_utf8_ranges() {
    let start = vec![0u8, 1u8];
    let end = vec![255u8, 254u8];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let slice = utf8_sequence.as_slice();
}

#[test]
fn test_as_slice_two_utf8_ranges_boundary() {
    let start = vec![0u8, 255u8];
    let end = vec![255u8, 255u8];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let slice = utf8_sequence.as_slice();
}

#[test]
fn test_as_slice_two_utf8_ranges_min_max() {
    let start = vec![128u8, 64u8];
    let end = vec![200u8, 255u8];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let slice = utf8_sequence.as_slice();
}

