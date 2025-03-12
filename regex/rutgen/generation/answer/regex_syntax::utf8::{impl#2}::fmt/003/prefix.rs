// Answer 0

#[test]
fn test_utf8_sequence_two() {
    let utf8_range1 = Utf8Range { start: 0, end: 127 };
    let utf8_range2 = Utf8Range { start: 128, end: 255 };
    let sequence = Utf8Sequence::Two([utf8_range1, utf8_range2]);
    let _ = format!("{:?}", sequence);
}

#[test]
fn test_utf8_sequence_two_boundary() {
    let utf8_range1 = Utf8Range { start: 127, end: 127 };
    let utf8_range2 = Utf8Range { start: 128, end: 128 };
    let sequence = Utf8Sequence::Two([utf8_range1, utf8_range2]);
    let _ = format!("{:?}", sequence);
}

#[test]
fn test_utf8_sequence_two_invalid() {
    let utf8_range1 = Utf8Range { start: 200, end: 220 };
    let utf8_range2 = Utf8Range { start: 221, end: 255 };
    let sequence = Utf8Sequence::Two([utf8_range1, utf8_range2]);
    let _ = format!("{:?}", sequence);
}

#[test]
fn test_utf8_sequence_two_maximum() {
    let utf8_range1 = Utf8Range { start: 255, end: 255 };
    let utf8_range2 = Utf8Range { start: 255, end: 255 };
    let sequence = Utf8Sequence::Two([utf8_range1, utf8_range2]);
    let _ = format!("{:?}", sequence);
}

