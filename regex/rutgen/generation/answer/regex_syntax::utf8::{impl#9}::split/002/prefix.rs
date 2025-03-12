// Answer 0

#[test]
fn test_split_end_equals_d7ff() {
    let range = ScalarRange { start: 0x0000, end: 0xD7FF };
    let result = range.split();
}

#[test]
fn test_split_mid_range() {
    let range = ScalarRange { start: 0x7FFF, end: 0xD7FF };
    let result = range.split();
}

#[test]
fn test_split_near_upper_bound() {
    let range = ScalarRange { start: 0xD7FE, end: 0xD7FF };
    let result = range.split();
}

#[test]
fn test_split_lower_bound() {
    let range = ScalarRange { start: 0x0000, end: 0xD7FF };
    let result = range.split();
}

#[test]
fn test_split_exceeding_zero() {
    let range = ScalarRange { start: 0xD000, end: 0xD7FF };
    let result = range.split();
}

