// Answer 0

#[test]
fn test_reverse_one_byte_range_valid() {
    let mut seq = Utf8Sequence::One(Utf8Range { start: 0x00, end: 0x7F });
    seq.reverse();
}

#[test]
fn test_reverse_one_byte_range_boundary_min() {
    let mut seq = Utf8Sequence::One(Utf8Range { start: 0x00, end: 0x00 });
    seq.reverse();
}

#[test]
fn test_reverse_one_byte_range_boundary_max() {
    let mut seq = Utf8Sequence::One(Utf8Range { start: 0x7F, end: 0x7F });
    seq.reverse();
}

