// Answer 0

#[test]
fn test_write_to_buffer_too_small_case_1() {
    let special = Special {
        max: StateID(0),
        quit_id: StateID(1),
        min_match: StateID(2),
        max_match: StateID(3),
        min_accel: StateID(4),
        max_accel: StateID(5),
        min_start: StateID(6),
        max_start: StateID(7),
    };
    let mut dst: [u8; 7] = [0; 7];
    let _ = special.write_to::<EndianType>(&mut dst);
}

#[test]
fn test_write_to_buffer_too_small_case_2() {
    let special = Special {
        max: StateID(8),
        quit_id: StateID(9),
        min_match: StateID(10),
        max_match: StateID(11),
        min_accel: StateID(12),
        max_accel: StateID(13),
        min_start: StateID(14),
        max_start: StateID(15),
    };
    let mut dst: [u8; 6] = [0; 6];
    let _ = special.write_to::<EndianType>(&mut dst);
}

#[test]
fn test_write_to_buffer_too_small_case_3() {
    let special = Special {
        max: StateID(16),
        quit_id: StateID(17),
        min_match: StateID(18),
        max_match: StateID(19),
        min_accel: StateID(20),
        max_accel: StateID(21),
        min_start: StateID(22),
        max_start: StateID(23),
    };
    let mut dst: [u8; 5] = [0; 5];
    let _ = special.write_to::<EndianType>(&mut dst);
}

#[test]
fn test_write_to_buffer_too_small_case_4() {
    let special = Special {
        max: StateID(24),
        quit_id: StateID(25),
        min_match: StateID(26),
        max_match: StateID(27),
        min_accel: StateID(28),
        max_accel: StateID(29),
        min_start: StateID(30),
        max_start: StateID(31),
    };
    let mut dst: [u8; 0] = [];
    let _ = special.write_to::<EndianType>(&mut dst);
}

