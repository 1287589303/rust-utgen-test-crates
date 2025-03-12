// Answer 0

#[test]
fn test_write_to_with_exact_buffer_size() {
    struct LittleEndian;
    impl crate::util::wire::Endian for LittleEndian {
        fn write_u128(value: u128, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }

    let mut byte_set = ByteSet::default();
    byte_set.add(1);
    byte_set.add(2);

    let mut dst = [0u8; 32];
    let result = byte_set.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_with_non_empty_byte_set() {
    struct BigEndian;
    impl crate::util::wire::Endian for BigEndian {
        fn write_u128(value: u128, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_be_bytes());
        }
    }

    let mut byte_set = ByteSet::default();
    byte_set.add(255);
    byte_set.add(128);

    let mut dst = [0u8; 32];
    let result = byte_set.write_to::<BigEndian>(&mut dst);
}

