// Answer 0

#[test]
fn test_write_to_anchored() {
    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst[0] = (value & 0xFF) as u8;
            dst[1] = ((value >> 8) & 0xFF) as u8;
            dst[2] = ((value >> 16) & 0xFF) as u8;
            dst[3] = ((value >> 24) & 0xFF) as u8;
        }
    }

    let start_kind = StartKind::Anchored;
    let nwrite = start_kind.write_to_len();
    let mut dst = vec![0u8; nwrite];
    
    let result = start_kind.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_both() {
    struct BigEndian;
    impl Endian for BigEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst[0] = ((value >> 24) & 0xFF) as u8;
            dst[1] = ((value >> 16) & 0xFF) as u8;
            dst[2] = ((value >> 8) & 0xFF) as u8;
            dst[3] = (value & 0xFF) as u8;
        }
    }

    let start_kind = StartKind::Both;
    let nwrite = start_kind.write_to_len();
    let mut dst = vec![0u8; nwrite];
    
    let result = start_kind.write_to::<BigEndian>(&mut dst);
}

