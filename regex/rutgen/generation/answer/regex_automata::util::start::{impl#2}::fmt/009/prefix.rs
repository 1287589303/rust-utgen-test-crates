// Answer 0

#[test]
fn test_fmt_initially_valid_state() {
    let start_byte_map = StartByteMap {
        map: [Start::NonWordByte; 256],
    };
    let mut buffer = Vec::new();
    let _ = write!(&mut buffer, "StartByteMap{{");
}

#[test]
fn test_fmt_single_iteration() {
    let start_byte_map = StartByteMap {
        map: [Start::NonWordByte; 256],
    };
    let mut buffer = Vec::new();
    let _ = write!(&mut buffer, "StartByteMap{{");
    for byte in 0..=255 {
        if byte > 0 {
            let _ = write!(&mut buffer, ", ");
        }
        let start = start_byte_map.map[usize::from(byte)];
        let _ = write!(&mut buffer, "{:?} => {:?}", DebugByte(byte), start);
        if byte == 0 {
            break; // effectively makes the for loop false
        }
    }
}

#[test]
fn test_fmt_with_error_on_closing() {
    let start_byte_map = StartByteMap {
        map: [Start::NonWordByte; 256],
    };
    let mut buffer = Vec::new();
    let _ = write!(&mut buffer, "StartByteMap{{");
    // Introducing an artificial error condition
    let invalid_bytes = [Start::NonWordByte; 255];
    for byte in 0..255 {
        let start = invalid_bytes[usize::from(byte)];
        let _ = write!(&mut buffer, "{:?} => {:?}", DebugByte(byte), start);
    }
    // Now attempting to close the buffer incorrectly to force an error
    let result = write!(&mut buffer, "}}").err(); // to force an error
}

