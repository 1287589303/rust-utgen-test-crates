// Answer 0

#[test]
fn test_internal_encode_empty_input() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder { 
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
    };
    let input: &[u8] = b""; // empty input
    let mut output = [0u8; 100]; // larger enough output buffer
    let output_index = encoder.internal_encode(input, &mut output);
    assert_eq!(output_index, 0);
}

#[test]
fn test_internal_encode_single_byte() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder { 
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
    };
    let input: &[u8] = b"A"; // single byte input
    let mut output = [0u8; 100]; // larger enough output buffer
    let output_index = encoder.internal_encode(input, &mut output);
    assert_eq!(output_index, 2); // output should have 2 bytes
    assert_eq!(&output[..output_index], b"QU"); // "A" encodes to "QU"
}

#[test]
fn test_internal_encode_two_bytes() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder { 
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
    };
    let input: &[u8] = b"AB"; // two bytes input
    let mut output = [0u8; 100]; // larger enough output buffer
    let output_index = encoder.internal_encode(input, &mut output);
    assert_eq!(output_index, 3); // output should have 3 bytes
    assert_eq!(&output[..output_index], b"QUI"); // "AB" encodes to "QUI"
}

#[test]
fn test_internal_encode_three_bytes() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder { 
        encode_table: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" 
    };
    let input: &[u8] = b"ABC"; // three bytes input
    let mut output = [0u8; 100]; // larger enough output buffer
    let output_index = encoder.internal_encode(input, &mut output);
    assert_eq!(output_index, 4); // output should have 4 bytes
    assert_eq!(&output[..output_index], b"QUJD"); // "ABC" encodes to "QUJD"
}

