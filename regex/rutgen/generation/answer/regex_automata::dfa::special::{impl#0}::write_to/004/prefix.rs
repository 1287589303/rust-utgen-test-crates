// Answer 0

#[test]
fn test_write_to_with_exact_buffer_size() {
    use crate::util::wire::Endian;

    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst[0] = (value & 0xFF) as u8;
            dst[1] = ((value >> 8) & 0xFF) as u8;
            dst[2] = ((value >> 16) & 0xFF) as u8;
            dst[3] = ((value >> 24) & 0xFF) as u8;
        }
    }

    let mut special = Special {
        max: StateID(1),
        quit_id: StateID(2),
        min_match: StateID(3),
        max_match: StateID(4),
        min_accel: StateID(5),
        max_accel: StateID(6),
        min_start: StateID(7),
        max_start: StateID(8),
    };

    let buffer_size = special.write_to_len();
    let mut buffer = vec![0u8; buffer_size];

    let _ = special.write_to::<LittleEndian>(&mut buffer);
}

#[test]
fn test_write_to_with_different_state_ids() {
    use crate::util::wire::Endian;

    struct LittleEndian;
    impl Endian for LittleEndian {
        fn write_u32(value: u32, dst: &mut [u8]) {
            dst[0] = (value & 0xFF) as u8;
            dst[1] = ((value >> 8) & 0xFF) as u8;
            dst[2] = ((value >> 16) & 0xFF) as u8;
            dst[3] = ((value >> 24) & 0xFF) as u8;
        }
    }

    let mut special = Special {
        max: StateID(10),
        quit_id: StateID(20),
        min_match: StateID(30),
        max_match: StateID(40),
        min_accel: StateID(50),
        max_accel: StateID(60),
        min_start: StateID(70),
        max_start: StateID(80),
    };

    let buffer_size = special.write_to_len();
    let mut buffer = vec![0u8; buffer_size];

    let _ = special.write_to::<LittleEndian>(&mut buffer);
}

