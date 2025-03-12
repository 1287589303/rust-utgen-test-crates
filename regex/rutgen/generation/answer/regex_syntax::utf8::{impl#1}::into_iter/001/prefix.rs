// Answer 0

#[test]
fn test_into_iter_one() {
    let utf8_sequence = Utf8Sequence::One(Utf8Range { start: 0, end: 0 });
    let mut iter = utf8_sequence.into_iter();
    let _ = iter.next();
}

#[test]
fn test_into_iter_two() {
    let utf8_sequence = Utf8Sequence::Two([
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
    ]);
    let mut iter = utf8_sequence.into_iter();
    let _ = iter.next();
    let _ = iter.next();
}

#[test]
fn test_into_iter_three() {
    let utf8_sequence = Utf8Sequence::Three([
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
    ]);
    let mut iter = utf8_sequence.into_iter();
    let _ = iter.next();
    let _ = iter.next();
    let _ = iter.next();
}

#[test]
fn test_into_iter_four() {
    let utf8_sequence = Utf8Sequence::Four([
        Utf8Range { start: 1, end: 1 },
        Utf8Range { start: 2, end: 2 },
        Utf8Range { start: 3, end: 3 },
        Utf8Range { start: 4, end: 4 },
    ]);
    let mut iter = utf8_sequence.into_iter();
    let _ = iter.next();
    let _ = iter.next();
    let _ = iter.next();
    let _ = iter.next();
}

#[test]
fn test_into_iter_empty() {
    let utf8_sequence = Utf8Sequence::One(Utf8Range { start: 0, end: 0 });
    let mut iter: core::slice::Iter<'_, Utf8Range> = utf8_sequence.as_slice().iter();
    assert!(iter.next().is_some());
}

