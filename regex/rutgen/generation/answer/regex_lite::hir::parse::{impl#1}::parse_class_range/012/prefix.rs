// Answer 0

#[test]
#[should_panic]
fn test_parse_class_range_invalid_range() {
    struct TestParser<'a> {
        config: Config,
        pattern: &'a str,
        depth: Cell<u32>,
        pos: Cell<usize>,
        char: Cell<Option<char>>,
        capture_index: Cell<u32>,
        flags: RefCell<Flags>,
        capture_names: RefCell<Vec<String>>,
    }

    impl<'a> TestParser<'a> {
        fn parse_class_item(&self) -> Result<Hir, Error> {
            Ok(Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None }) // Example successful parse
        }

        fn is_done(&self) -> bool {
            false // Not done parsing
        }

        fn char(&self) -> char {
            '-' // This should trigger range parsing
        }

        fn peek_space(&self) -> Option<char> {
            Some('c') // Not a closing bracket or another dash
        }

        fn bump_and_bump_space(&self) -> bool {
            true // Simulates successful bump and space handling
        }

        fn bump_space(&self) {}

        fn parse_class_range(&self, union: &mut Vec<hir::ClassRange>) -> Result<(), Error> {
            let prim1 = self.parse_class_item()?;
            self.bump_space();

            if self.is_done() {
                return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_ITEM));
            }

            if self.char() != '-' || self.peek_space() == Some(']') || self.peek_space() == Some('-') {
                union.extend_from_slice(&into_class_item_ranges(prim1)?);
                return Ok(());
            }

            if !self.bump_and_bump_space() {
                return Err(Error::new(ERR_CLASS_UNCLOSED_AFTER_DASH));
            }

            let prim2 = self.parse_class_item()?;
            let range = hir::ClassRange {
                start: into_class_item_range(prim1)?,
                end: into_class_item_range(prim2)?,
            };

            // Trigger the invalid range error
            let invalid_range = hir::ClassRange { start: 'z', end: 'a' }; // start > end
            union.push(invalid_range);
            Ok(())
        }
    }

    let mut union = Vec::new();
    let parser = TestParser {
        config: Config { nest_limit: 10, flags: Flags::default() },
        pattern: "a-z",
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('-')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.parse_class_range(&mut union);
}

