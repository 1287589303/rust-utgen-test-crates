// Answer 0

#[test]
fn test_general_purpose_new() {
    struct Alphabet {
        symbols: Vec<u8>,
    }

    struct GeneralPurposeConfig {
        padding: bool,
    }

    struct GeneralPurpose {
        encode_table: Vec<u8>,
        decode_table: Vec<u8>,
        config: GeneralPurposeConfig,
    }

    impl GeneralPurpose {
        pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self {
            Self {
                encode_table: alphabet.symbols.clone(),
                decode_table: alphabet.symbols.clone(),
                config,
            }
        }
    }

    let alphabet = Alphabet {
        symbols: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_vec(),
    };
    
    let config = GeneralPurposeConfig {
        padding: true,
    };

    let engine = GeneralPurpose::new(&alphabet, config);
    
    assert_eq!(engine.encode_table.len(), alphabet.symbols.len());
    assert_eq!(engine.decode_table.len(), alphabet.symbols.len());
    assert_eq!(engine.config.padding, true);
}

#[test]
fn test_general_purpose_new_empty_alphabet() {
    struct Alphabet {
        symbols: Vec<u8>,
    }

    struct GeneralPurposeConfig {
        padding: bool,
    }

    struct GeneralPurpose {
        encode_table: Vec<u8>,
        decode_table: Vec<u8>,
        config: GeneralPurposeConfig,
    }

    impl GeneralPurpose {
        pub const fn new(alphabet: &Alphabet, config: GeneralPurposeConfig) -> Self {
            Self {
                encode_table: alphabet.symbols.clone(),
                decode_table: alphabet.symbols.clone(),
                config,
            }
        }
    }

    let empty_alphabet = Alphabet {
        symbols: Vec::new(),
    };
    
    let config = GeneralPurposeConfig {
        padding: false,
    };

    let engine = GeneralPurpose::new(&empty_alphabet, config);
    
    assert_eq!(engine.encode_table.len(), 0);
    assert_eq!(engine.decode_table.len(), 0);
    assert_eq!(engine.config.padding, false);
}

