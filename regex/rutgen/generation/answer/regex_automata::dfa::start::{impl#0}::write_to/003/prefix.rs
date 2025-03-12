// Answer 0

#[test]
fn test_write_to_unanchored_success() {
    struct LittleEndian;

    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst[0] = (value & 0xFF) as u8;
            dst[1] = ((value >> 8) & 0xFF) as u8;
            dst[2] = ((value >> 16) & 0xFF) as u8;
            dst[3] = ((value >> 24) & 0xFF) as u8;
        }
    }

    let start_kind = StartKind::Unanchored;
    let mut dst = [0u8; 4];
    let result = start_kind.write_to::<LittleEndian>(&mut dst);
} 

#[test]
fn test_write_to_both_success() {
    struct LittleEndian;

    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst[0] = (value & 0xFF) as u8;
            dst[1] = ((value >> 8) & 0xFF) as u8;
            dst[2] = ((value >> 16) & 0xFF) as u8;
            dst[3] = ((value >> 24) & 0xFF) as u8;
        }
    }

    let start_kind = StartKind::Both;
    let mut dst = [0u8; 4];
    let result = start_kind.write_to::<LittleEndian>(&mut dst);
}

#[test]
fn test_write_to_anchored_success() {
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
    let mut dst = [0u8; 4];
    let result = start_kind.write_to::<LittleEndian>(&mut dst);
}

