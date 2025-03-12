// Answer 0

#[test]
fn test_write_to_all_flags_false() {
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    let mut dst = [0u8; 4];
    let result = flags.write_to::<Endian>(&mut dst);
}

#[test]
fn test_write_to_has_empty_true() {
    let flags = Flags {
        has_empty: true,
        is_utf8: false,
        is_always_start_anchored: false,
    };
    let mut dst = [0u8; 4];
    let result = flags.write_to::<Endian>(&mut dst);
}

#[test]
fn test_write_to_is_utf8_true() {
    let flags = Flags {
        has_empty: false,
        is_utf8: true,
        is_always_start_anchored: false,
    };
    let mut dst = [0u8; 4];
    let result = flags.write_to::<Endian>(&mut dst);
}

#[test]
fn test_write_to_is_always_start_anchored_true() {
    let flags = Flags {
        has_empty: false,
        is_utf8: false,
        is_always_start_anchored: true,
    };
    let mut dst = [0u8; 4];
    let result = flags.write_to::<Endian>(&mut dst);
}

#[test]
fn test_write_to_all_flags_true() {
    let flags = Flags {
        has_empty: true,
        is_utf8: true,
        is_always_start_anchored: true,
    };
    let mut dst = [0u8; 4];
    let result = flags.write_to::<Endian>(&mut dst);
}

