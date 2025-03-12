// Answer 0

#[test]
pub(crate) fn test_write_to_success() {
    let mut byte_set = ByteSet::default();
    let mut dst = [0u8; 16];
    let nwrite = byte_set.write_to_len();
    let result = byte_set.write_to::<crate::util::wire::LittleEndian>(&mut dst);
}

#[test]
#[should_panic]
pub(crate) fn test_write_to_failure() {
    let byte_set = ByteSet::default();
    let mut dst = [0u8; 15];
    let result = byte_set.write_to::<crate::util::wire::LittleEndian>(&mut dst);
}

#[test]
pub(crate) fn test_write_to_success_boundaries() {
    let byte_set = ByteSet::default();
    let mut dst = [0u8; 16];
    let result = byte_set.write_to::<crate::util::wire::LittleEndian>(&mut dst);
    let nwrite = byte_set.write_to_len();
}

#[test]
#[should_panic]
pub(crate) fn test_write_to_failure_boundaries() {
    let byte_set = ByteSet::default();
    let mut dst = [0u8; 14];
    let result = byte_set.write_to::<crate::util::wire::LittleEndian>(&mut dst);
}

