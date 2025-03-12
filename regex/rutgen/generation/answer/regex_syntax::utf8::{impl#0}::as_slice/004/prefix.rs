// Answer 0

#[test]
fn test_as_slice_one() {
    let range = Utf8Range { start: 0, end: 0 };
    let sequence = Utf8Sequence::One(range);
    let slice = sequence.as_slice();
}

#[test]
fn test_as_slice_one_boundary() {
    let range_start = Utf8Range { start: 0, end: 255 };
    let sequence_start = Utf8Sequence::One(range_start);
    let slice_start = sequence_start.as_slice();
    
    let range_end = Utf8Range { start: 255, end: 255 };
    let sequence_end = Utf8Sequence::One(range_end);
    let slice_end = sequence_end.as_slice();
}

