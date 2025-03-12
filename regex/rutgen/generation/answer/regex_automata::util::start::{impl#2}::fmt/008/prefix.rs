// Answer 0

#[test]
fn test_start_byte_map_debug_empty() {
    let start_byte_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let _ = core::fmt::Debug::fmt(&start_byte_map, &mut core::fmt::Formatter::new());
}

#[test]
fn test_start_byte_map_debug_single_zero() {
    let mut map = [Start::NonWordByte; 256];
    map[0] = Start::Text; // Initialize start for byte 0
    let start_byte_map = StartByteMap { map };
    let _ = core::fmt::Debug::fmt(&start_byte_map, &mut core::fmt::Formatter::new());
}

#[test]
fn test_start_byte_map_debug_full_range() {
    let mut map = [Start::NonWordByte; 256];
    for byte in 0..=255 {
        map[byte] = match byte {
            0 => Start::Text,
            1 => Start::WordByte,
            2 => Start::LineLF,
            3 => Start::LineCR,
            _ => Start::CustomLineTerminator,
        };
    }
    let start_byte_map = StartByteMap { map };
    let _ = core::fmt::Debug::fmt(&start_byte_map, &mut core::fmt::Formatter::new());
}

