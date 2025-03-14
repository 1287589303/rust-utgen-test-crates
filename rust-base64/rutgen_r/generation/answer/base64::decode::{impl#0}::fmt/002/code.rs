// Answer 0

#[test]
fn test_invalid_last_symbol_fmt() {
    struct InvalidLastSymbolFormatter {
        index: usize,
        byte: u8,
    }

    impl std::fmt::Display for InvalidLastSymbolFormatter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid last symbol {}, offset {}.", self.byte, self.index)
        }
    }

    let symbol = InvalidLastSymbolFormatter { index: 5, byte: b'A' };
    let expected_output = "Invalid last symbol A, offset 5.";
    let result = format!("{}", symbol);
    assert_eq!(result, expected_output);
}

#[test]
fn test_invalid_last_symbol_fmt_another_case() {
    struct InvalidLastSymbolFormatter {
        index: usize,
        byte: u8,
    }

    impl std::fmt::Display for InvalidLastSymbolFormatter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid last symbol {}, offset {}.", self.byte, self.index)
        }
    }

    let symbol = InvalidLastSymbolFormatter { index: 10, byte: b'Z' };
    let expected_output = "Invalid last symbol Z, offset 10.";
    let result = format!("{}", symbol);
    assert_eq!(result, expected_output);
}

