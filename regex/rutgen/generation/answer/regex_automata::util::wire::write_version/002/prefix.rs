// Answer 0

#[test]
fn test_write_version_exact_buffer_size() {
    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }

    let version: u32 = 0; // test with the minimum valid version number
    let mut buffer = [0u8; 4]; // buffer size equals size_of::<u32>()
    let result = write_version::<LittleEndian>(version, &mut buffer);
}

#[test]
fn test_write_version_max_valid() {
    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }

    let version: u32 = u32::MAX; // test with the maximum valid version number
    let mut buffer = [0u8; 4]; // buffer size equals size_of::<u32>()
    let result = write_version::<LittleEndian>(version, &mut buffer);
}

#[test]
fn test_write_version_mid_value() {
    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
    }

    let version: u32 = 1_000_000; // test with a mid-range version number
    let mut buffer = [0u8; 4]; // buffer size equals size_of::<u32>()
    let result = write_version::<LittleEndian>(version, &mut buffer);
}

