// Answer 0

#[test]
fn test_matches_with_one_utf8_range() {
    let start = vec![0b00000000];
    let end = vec![0b00000001];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let bytes = vec![0b00000000];
    let result = utf8_sequence.matches(&bytes);
}

#[test]
fn test_matches_with_two_utf8_ranges() {
    let start = vec![0b00000000, 0b00000010];
    let end = vec![0b00000001, 0b00000011];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let bytes = vec![0b00000000, 0b00000010];
    let result = utf8_sequence.matches(&bytes);
}

#[test]
fn test_matches_with_three_utf8_ranges() {
    let start = vec![0b00000000, 0b00000100, 0b00001000];
    let end = vec![0b00000001, 0b00000101, 0b00001001];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let bytes = vec![0b00000000, 0b00000100, 0b00001000];
    let result = utf8_sequence.matches(&bytes);
}

#[test]
fn test_matches_with_four_utf8_ranges() {
    let start = vec![0b00000000, 0b00000010, 0b00000100, 0b00001000];
    let end = vec![0b00000001, 0b00000011, 0b00000101, 0b00001001];
    let utf8_sequence = Utf8Sequence::from_encoded_range(&start, &end);
    let bytes = vec![0b00000000, 0b00000010, 0b00000100, 0b00001000];
    let result = utf8_sequence.matches(&bytes);
}

