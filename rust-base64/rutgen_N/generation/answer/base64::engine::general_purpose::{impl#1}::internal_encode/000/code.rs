// Answer 0

#[test]
fn test_internal_encode_empty_input() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec().try_into().unwrap(),
    };

    let input: &[u8] = &[];
    let mut output = [0u8; 0];
    let result = encoder.internal_encode(input, &mut output);
    assert_eq!(result, 0);
}

#[test]
fn test_internal_encode_single_byte() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec().try_into().unwrap(),
    };

    let input: &[u8] = &[0b00000001];
    let mut output = [0u8; 4];
    let result = encoder.internal_encode(input, &mut output);
    assert_eq!(result, 2);
    assert_eq!(&output[..result], b"AA==");
}

#[test]
fn test_internal_encode_two_bytes() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec().try_into().unwrap(),
    };

    let input: &[u8] = &[0b00000001, 0b00000010];
    let mut output = [0u8; 4];
    let result = encoder.internal_encode(input, &mut output);
    assert_eq!(result, 3);
    assert_eq!(&output[..result], b"AQI=");
}

#[test]
fn test_internal_encode_three_bytes() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec().try_into().unwrap(),
    };

    let input: &[u8] = &[0b00000001, 0b00000010, 0b00000011];
    let mut output = [0u8; 4];
    let result = encoder.internal_encode(input, &mut output);
    assert_eq!(result, 4);
    assert_eq!(&output[..result], b"AQID");
}

#[test]
fn test_internal_encode_multiple_blocks() {
    struct Encoder {
        encode_table: [u8; 64],
    }

    let encoder = Encoder {
        encode_table: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec().try_into().unwrap(),
    };

    let input: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut output = [0u8; 16];
    let result = encoder.internal_encode(input, &mut output);
    assert_eq!(result, 16);
    assert_eq!(&output[..result], b"AAECAwQFBgcICQ==");
}

