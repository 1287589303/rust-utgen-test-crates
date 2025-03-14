// Answer 0

#[test]
fn test_fmt_reserved_byte() {
    struct AlphabetError {
        kind: AlphabetErrorKind,
    }

    enum AlphabetErrorKind {
        ReservedByte(u8),
    }

    impl std::fmt::Debug for AlphabetError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.kind {
                AlphabetErrorKind::ReservedByte(b) => write!(f, "Reserved byte: {:#04x}", b),
            }
        }
    }

    let error = AlphabetError {
        kind: AlphabetErrorKind::ReservedByte(0xFF),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "Reserved byte: 0xff");
}

