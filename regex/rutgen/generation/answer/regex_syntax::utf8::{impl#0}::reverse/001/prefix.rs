// Answer 0

#[test]
fn test_reverse_four_ranges_normal() {
    let mut sequence = Utf8Sequence::Four([
        Utf8Range { start: 0, end: 0 },
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
    ]);
    sequence.reverse();
}

#[test]
fn test_reverse_four_ranges_max() {
    let mut sequence = Utf8Sequence::Four([
        Utf8Range { start: 252, end: 252 },
        Utf8Range { start: 253, end: 253 },
        Utf8Range { start: 254, end: 254 },
        Utf8Range { start: 255, end: 255 },
    ]);
    sequence.reverse();
}

#[test]
fn test_reverse_four_ranges_mixed() {
    let mut sequence = Utf8Sequence::Four([
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 30, end: 40 },
        Utf8Range { start: 50, end: 60 },
        Utf8Range { start: 70, end: 80 },
    ]);
    sequence.reverse();
}

#[test]
fn test_reverse_four_ranges_boundaries() {
    let mut sequence = Utf8Sequence::Four([
        Utf8Range { start: 0, end: 255 },
        Utf8Range { start: 1, end: 255 },
        Utf8Range { start: 2, end: 255 },
        Utf8Range { start: 3, end: 255 },
    ]);
    sequence.reverse();
}

#[test]
fn test_reverse_four_ranges_consecutive() {
    let mut sequence = Utf8Sequence::Four([
        Utf8Range { start: 5, end: 8 },
        Utf8Range { start: 9, end: 12 },
        Utf8Range { start: 13, end: 16 },
        Utf8Range { start: 17, end: 20 },
    ]);
    sequence.reverse();
}

