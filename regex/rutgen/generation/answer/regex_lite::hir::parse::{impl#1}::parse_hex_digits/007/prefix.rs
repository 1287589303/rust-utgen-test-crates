// Answer 0

#[test]
fn test_parse_hex_digits_invalid_digit() {
    struct TestParser {
        pos: Cell<usize>,
        char: Cell<Option<char>>,
    }

    impl<'a> Parser<'a> {
        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(self.pos.get() + 1);
            true
        }
        
        fn char(&self) -> char {
            self.char.get().unwrap_or('\0')
        }

        fn hir_char(&self, ch: char) -> Hir {
            Hir {
                kind: HirKind::Char,
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            }
        }
    }

    let parser = TestParser {
        pos: Cell::new(0),
        char: Cell::new(Some('g')), // 'g' is a non-hex digit
    };

    let result_2 = parser.parse_hex_digits(2);
    let result_4 = parser.parse_hex_digits(4);
    let result_8 = parser.parse_hex_digits(8);
}

#[test]
fn test_parse_hex_digits_empty_input() {
    struct TestParser {
        pos: Cell<usize>,
        char: Cell<Option<char>>,
    }

    impl<'a> Parser<'a> {
        fn bump_and_bump_space(&self) -> bool {
            self.pos.set(self.pos.get() + 1);
            true
        }

        fn char(&self) -> char {
            self.char.get().unwrap_or('\0')
        }

        fn hir_char(&self, ch: char) -> Hir {
            Hir {
                kind: HirKind::Char,
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            }
        }
    }

    let parser = TestParser {
        pos: Cell::new(0),
        char: Cell::new(Some('g')), // 'g' is a non-hex digit
    };

    let result_2 = parser.parse_hex_digits(2);
    let result_4 = parser.parse_hex_digits(4);
    let result_8 = parser.parse_hex_digits(8);
}

