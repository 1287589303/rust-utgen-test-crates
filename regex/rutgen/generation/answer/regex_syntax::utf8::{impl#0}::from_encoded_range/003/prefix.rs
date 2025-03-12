// Answer 0

#[test]
fn test_from_encoded_range_three_elements() {
    let start: &[u8] = &[0, 127, 192];
    let end: &[u8] = &[127, 191, 223];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

#[test]
fn test_from_encoded_range_three_elements_with_edge_values() {
    let start: &[u8] = &[1, 126, 193];
    let end: &[u8] = &[126, 192, 223];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

#[test]
fn test_from_encoded_range_three_elements_at_bounds() {
    let start: &[u8] = &[0, 127, 255];
    let end: &[u8] = &[127, 255, 255];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

#[test]
#[should_panic]
fn test_from_encoded_range_three_elements_invalid() {
    let start: &[u8] = &[0, 127];
    let end: &[u8] = &[127, 191];
    let result = Utf8Sequence::from_encoded_range(start, end);
}

