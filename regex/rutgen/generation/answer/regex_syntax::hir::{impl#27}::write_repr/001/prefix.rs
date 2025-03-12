// Answer 0

#[test]
fn test_write_repr_zeroed_lookset() {
    let lookset = LookSet { bits: 0 };
    let mut slice = [0u8; 4];
    lookset.write_repr(&mut slice);
}

#[test]
fn test_write_repr_full_lookset() {
    let lookset = LookSet { bits: u32::MAX };
    let mut slice = [0u8; 4];
    lookset.write_repr(&mut slice);
}

#[test]
fn test_write_repr_mid_range_lookset() {
    let lookset = LookSet { bits: 1234567890 };
    let mut slice = [0u8; 4];
    lookset.write_repr(&mut slice);
}

#[test]
#[should_panic]
fn test_write_repr_insufficient_slice_length() {
    let lookset = LookSet { bits: 1234567890 };
    let mut slice = [0u8; 3];
    lookset.write_repr(&mut slice);
}

