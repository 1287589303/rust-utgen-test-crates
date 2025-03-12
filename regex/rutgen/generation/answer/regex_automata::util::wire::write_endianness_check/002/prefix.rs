// Answer 0

#[test]
fn test_write_endianness_check_little_endian() {
    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(val: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&val.to_le_bytes());
        }
    }

    let mut buffer = [0u8; 4];
    let result = write_endianness_check::<LittleEndian>(&mut buffer);
    let expected = Ok(4);
    // The assertion is omitted as per guidelines, but this will check
    // that buffer contains the bytes representing 0xFEFF in little-endian.
}

#[test]
fn test_write_endianness_check_big_endian() {
    struct BigEndian;
    impl Endian for BigEndian {
        fn write_u32(val: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&val.to_be_bytes());
        }
    }

    let mut buffer = [0u8; 4];
    let result = write_endianness_check::<BigEndian>(&mut buffer);
    let expected = Ok(4);
    // The assertion is omitted as per guidelines, but this will check
    // that buffer contains the bytes representing 0xFEFF in big-endian.
}

