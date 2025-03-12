// Answer 0

#[test]
fn test_write_pattern_id_le() {
    struct LE;
    impl Endian for LE {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }
    
    let pattern_id = PatternID(0x12345678.into());
    let mut dst = [0u8; 4];
    let result = write_pattern_id::<LE>(pattern_id, &mut dst);
}

#[test]
fn test_write_pattern_id_be() {
    struct BE;
    impl Endian for BE {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_be_bytes());
        }
    }
    
    let pattern_id = PatternID(0x12345678.into());
    let mut dst = [0u8; 4];
    let result = write_pattern_id::<BE>(pattern_id, &mut dst);
}

#[test]
fn test_write_pattern_id_le_boundary() {
    struct LE;
    impl Endian for LE {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }
    
    let pattern_id = PatternID(0u32.into());
    let mut dst = [0u8; 4];
    let result = write_pattern_id::<LE>(pattern_id, &mut dst);
}

#[test]
fn test_write_pattern_id_be_boundary() {
    struct BE;
    impl Endian for BE {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_be_bytes());
        }
    }
    
    let pattern_id = PatternID(u32::MAX.into());
    let mut dst = [0u8; 4];
    let result = write_pattern_id::<BE>(pattern_id, &mut dst);
}

