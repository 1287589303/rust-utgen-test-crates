// Answer 0

#[test]
fn test_general_purpose_new() {
    struct Alphabet {
        table: Vec<u8>,
    }

    struct GeneralPurposeConfig {
        padding: bool,
    }

    impl GeneralPurposeConfig {
        pub fn new(padding: bool) -> Self {
            Self { padding }
        }
    }

    struct GeneralPurpose {
        encode_table: Vec<u8>,
        decode_table: Vec<u8>,
        config: GeneralPurposeConfig,
    }

    fn encode_table(alphabet: &Alphabet) -> Vec<u8> {
        alphabet.table.clone()
    }

    fn decode_table(alphabet: &Alphabet) -> Vec<u8> {
        alphabet.table.clone().into_iter().rev().collect()
    }

    let alphabet = Alphabet { table: vec![b'A', b'B', b'C', b'D', b'E', b'F'] };
    let config = GeneralPurposeConfig::new(true);
    
    let engine = GeneralPurpose {
        encode_table: encode_table(&alphabet),
        decode_table: decode_table(&alphabet),
        config,
    };
    
    assert_eq!(engine.encode_table, vec![b'A', b'B', b'C', b'D', b'E', b'F']);
    assert_eq!(engine.decode_table, vec![b'F', b'E', b'D', b'C', b'B', b'A']);
    assert!(engine.config.padding);
}

#[test]
fn test_general_purpose_new_empty_alphabet() {
    struct Alphabet {
        table: Vec<u8>,
    }

    struct GeneralPurposeConfig {
        padding: bool,
    }

    impl GeneralPurposeConfig {
        pub fn new(padding: bool) -> Self {
            Self { padding }
        }
    }

    struct GeneralPurpose {
        encode_table: Vec<u8>,
        decode_table: Vec<u8>,
        config: GeneralPurposeConfig,
    }

    fn encode_table(alphabet: &Alphabet) -> Vec<u8> {
        alphabet.table.clone()
    }

    fn decode_table(alphabet: &Alphabet) -> Vec<u8> {
        alphabet.table.clone().into_iter().rev().collect()
    }

    let alphabet = Alphabet { table: vec![] };
    let config = GeneralPurposeConfig::new(false);
    
    let engine = GeneralPurpose {
        encode_table: encode_table(&alphabet),
        decode_table: decode_table(&alphabet),
        config,
    };
    
    assert_eq!(engine.encode_table, vec![]);
    assert_eq!(engine.decode_table, vec![]);
    assert!(!engine.config.padding);
}

