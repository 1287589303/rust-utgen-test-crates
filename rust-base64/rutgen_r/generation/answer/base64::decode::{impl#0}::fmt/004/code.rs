// Answer 0

#[test]
fn test_invalid_byte_fmt() {
    struct InvalidByteFmt {
        index: usize,
        byte: u8,
    }

    impl std::fmt::Debug for InvalidByteFmt {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid symbol {}, offset {}.", self.byte, self.index)
        }
    }

    let invalid_symbol = InvalidByteFmt { index: 5, byte: b'A' };
    let result = format!("{:?}", invalid_symbol);
    assert_eq!(result, "Invalid symbol 65, offset 5.");
}

#[test]
fn test_invalid_length_fmt() {
    struct InvalidLengthFmt {
        length: usize,
    }

    impl std::fmt::Debug for InvalidLengthFmt {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid input length: {}", self.length)
        }
    }

    let invalid_length = InvalidLengthFmt { length: 10 };
    let result = format!("{:?}", invalid_length);
    assert_eq!(result, "Invalid input length: 10");
}

#[test]
fn test_invalid_last_symbol_fmt() {
    struct InvalidLastSymbolFmt {
        index: usize,
        byte: u8,
    }

    impl std::fmt::Debug for InvalidLastSymbolFmt {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid last symbol {}, offset {}.", self.byte, self.index)
        }
    }

    let invalid_last = InvalidLastSymbolFmt { index: 3, byte: b'=' };
    let result = format!("{:?}", invalid_last);
    assert_eq!(result, "Invalid last symbol 61, offset 3.");
}

#[test]
fn test_invalid_padding_fmt() {
    struct InvalidPaddingFmt;

    impl std::fmt::Debug for InvalidPaddingFmt {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid padding")
        }
    }

    let invalid_padding = InvalidPaddingFmt;
    let result = format!("{:?}", invalid_padding);
    assert_eq!(result, "Invalid padding");
}

