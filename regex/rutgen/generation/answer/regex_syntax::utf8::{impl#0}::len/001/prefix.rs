// Answer 0

#[test]
fn test_len_one() {
    let sequence = Utf8Sequence::One(Utf8Range { start: 0, end: 127 });
    let result = sequence.len();
}

#[test]
fn test_len_two() {
    let sequence = Utf8Sequence::Two([
        Utf8Range { start: 128, end: 191 },
        Utf8Range { start: 192, end: 223 },
    ]);
    let result = sequence.len();
}

#[test]
fn test_len_three() {
    let sequence = Utf8Sequence::Three([
        Utf8Range { start: 224, end: 239 },
        Utf8Range { start: 240, end: 247 },
        Utf8Range { start: 248, end: 255 },
    ]);
    let result = sequence.len();
}

#[test]
fn test_len_four() {
    let sequence = Utf8Sequence::Four([
        Utf8Range { start: 0, end: 0 },
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
    ]);
    let result = sequence.len();
}

