// Answer 0

#[test]
fn test_start_byte_map_format() {
    let mut fmt_output = String::new();
    let start_byte_map = StartByteMap {
        map: [Start::NonWordByte; 256],
    };
    write!(&mut fmt_output, "StartByteMap{{").unwrap();
    for byte in 0..=255 {
        start_byte_map.map[usize::from(byte)] = Start::WordByte; // Setup all bytes as WordByte
        if byte > 0 {
            let result = write!(&mut fmt_output, ", ");
            if byte == 1 { // Simulate an error for byte 1
                assert!(result.is_err());
            }
        }
        let _ = write!(&mut fmt_output, "{:?} => {:?}", DebugByte(byte), start_byte_map.map[usize::from(byte)]);
    }
}

#[test]
fn test_start_byte_map_format_with_err_on_next_line() {
    let mut fmt_output = String::new();
    let start_byte_map = StartByteMap {
        map: [Start::WordByte; 256],
    };
    write!(&mut fmt_output, "StartByteMap{{").unwrap();
    for byte in 2..=255 {
        if byte > 0 {
            let result = write!(&mut fmt_output, ", ");
            if byte == 2 { // Simulate an error for byte 2
                assert!(result.is_err());
            }
        }
        let _ = write!(&mut fmt_output, "{:?} => {:?}", DebugByte(byte), start_byte_map.map[usize::from(byte)]);
    }
}

