// Answer 0

#[test]
fn test_parse_class_range_valid_first_half_invalid_second_half() {
    struct TestParser {
        config: Config,
        pattern: &'static str,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl<'a> TestParser {
        fn parse_class_item(&self) -> Result<Hir, Error> {
            // Simulate a valid character for the first half
            let ch = 'a';
            self.char.set(Some(ch));
            Ok(Hir { kind: HirKind::Char(ch), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None })
        }
        
        fn is_done(&self) -> bool {
            false // Not done parsing
        }

        fn char(&self) -> Option<char> {
            Some('-') // The character is '-'
        }

        fn peek_space(&self) -> Option<char> {
            None // Not ']' or '-'
        }

        fn bump_and_bump_space(&self) -> bool {
            true // Simulating a successful bump
        }

        fn bump_space(&self) {}
        
        fn parse_class_range(&self, union: &mut Vec<hir::ClassRange>) -> Result<(), Error> {
            // The actual parse_class_range call
            self.parse_class_range(union)
        }
    }

    let mut union = Vec::new();
    let parser = TestParser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "a-b",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_valid_first_half_invalid_second_half_no_valid_range() {
    struct TestParser {
        config: Config,
        pattern: &'static str,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl<'a> TestParser {
        fn parse_class_item(&self) -> Result<Hir, Error> {
            // Simulate a valid character for the first half
            let ch = 'a';
            self.char.set(Some(ch));
            Ok(Hir { kind: HirKind::Char(ch), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None })
        }
        
        fn is_done(&self) -> bool {
            false // Not done parsing
        }

        fn char(&self) -> Option<char> {
            Some('-') // The character is '-'
        }

        fn peek_space(&self) -> Option<char> {
            None // Not ']' or '-'
        }

        fn bump_and_bump_space(&self) -> bool {
            true // Simulating a successful bump
        }

        fn bump_space(&self) {}
        
        fn parse_class_range(&self, union: &mut Vec<hir::ClassRange>) -> Result<(), Error> {
            // The actual parse_class_range call
            self.parse_class_range(union)
        }
    }

    let mut union = Vec::new();
    let parser = TestParser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "a-*", // Simulate an invalid character after '-'
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(None),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.parse_class_range(&mut union);
}

