// Answer 0

#[test]
fn test_reverse_utf8_sequence_three() {
    let mut sequence = Utf8Sequence::Three([
        Utf8Range { start: 0x00, end: 0x01 },
        Utf8Range { start: 0x02, end: 0x03 },
        Utf8Range { start: 0x04, end: 0x04 },
    ]);
    sequence.reverse();
}

#[test]
fn test_reverse_utf8_sequence_three_boundary() {
    let mut sequence = Utf8Sequence::Three([
        Utf8Range { start: 0xFF, end: 0xFF },
        Utf8Range { start: 0x80, end: 0xBF },
        Utf8Range { start: 0xC0, end: 0xDF },
    ]);
    sequence.reverse();
}

#[test]
fn test_reverse_utf8_sequence_three_full_range() {
    let mut sequence = Utf8Sequence::Three([
        Utf8Range { start: 0x00, end: 0x7F },
        Utf8Range { start: 0x80, end: 0xBF },
        Utf8Range { start: 0xC0, end: 0xFF },
    ]);
    sequence.reverse();
}

