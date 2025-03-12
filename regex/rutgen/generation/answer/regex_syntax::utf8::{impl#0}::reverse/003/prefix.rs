// Answer 0

#[test]
fn test_reverse_two_ranges() {
    let mut utf8_sequence = Utf8Sequence::Two([
        Utf8Range { start: 0, end: 3 },
        Utf8Range { start: 1, end: 4 },
    ]);
    utf8_sequence.reverse();
}

#[test]
fn test_reverse_two_ranges_boundary() {
    let mut utf8_sequence = Utf8Sequence::Two([
        Utf8Range { start: 0, end: 0 },
        Utf8Range { start: 255, end: 255 },
    ]);
    utf8_sequence.reverse();
}

