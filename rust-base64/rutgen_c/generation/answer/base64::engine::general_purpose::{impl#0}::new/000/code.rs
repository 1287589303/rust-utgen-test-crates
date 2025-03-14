// Answer 0

#[test]
fn test_general_purpose_new() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    impl TestAlphabet {
        const fn new() -> Self {
            Self {
                symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            }
        }
    }

    let test_alphabet = TestAlphabet::new();
    let config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = GeneralPurpose::new(&test_alphabet, config);
    
    assert_eq!(engine.encode_table[0], b'A');
    assert_eq!(engine.decode_table[b'A' as usize], 0);
    
    assert_eq!(engine.config.encode_padding, true);
    assert_eq!(engine.config.decode_padding_mode, DecodePaddingMode::RequireNone);
}

#[test]
fn test_general_purpose_new_with_no_padding() {
    struct TestAlphabet {
        symbols: [u8; 64],
    }

    impl TestAlphabet {
        const fn new() -> Self {
            Self {
                symbols: *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
            }
        }
    }

    let test_alphabet = TestAlphabet::new();
    let config = GeneralPurposeConfig {
        encode_padding: false,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = GeneralPurpose::new(&test_alphabet, config);
    
    assert_eq!(engine.encode_table[0], b'A');
    assert_eq!(engine.decode_table[b'A' as usize], 0);
    
    assert_eq!(engine.config.encode_padding, false);
    assert_eq!(engine.config.decode_padding_mode, DecodePaddingMode::RequireNone);
}

