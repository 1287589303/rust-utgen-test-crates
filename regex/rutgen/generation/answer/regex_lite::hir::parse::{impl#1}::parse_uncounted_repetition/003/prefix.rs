// Answer 0

#[test]
fn test_parse_uncounted_repetition_plus() {
    let pattern = "+";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let sub_hir = Hir::char('a'); // Example sub expression
    let concat = vec![sub_hir]; // Ensure concat has at least one element

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('+')), // self.char() matches '+'
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { swap_greed: false, ..Flags::default() }), // swap_greed is false
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.parse_uncounted_repetition(concat); // Call the function under test
}

#[test]
fn test_parse_uncounted_repetition_star() {
    let pattern = "*";
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let sub_hir = Hir::char('b'); // Example sub expression
    let concat = vec![sub_hir]; // Ensure concat has at least one element

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('*')), // self.char() matches '*'
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { swap_greed: false, ..Flags::default() }), // swap_greed is false
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.parse_uncounted_repetition(concat); // Call the function under test
}

#[test]
fn test_parse_uncounted_repetition_question() {
    let pattern = "?"; 
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let sub_hir = Hir::char('c'); // Example sub expression
    let concat = vec![sub_hir]; // Ensure concat has at least one element

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('?')), // self.char() matches '?'
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { swap_greed: false, ..Flags::default() }), // swap_greed is false
        capture_names: RefCell::new(vec![]),
    };

    let _ = parser.parse_uncounted_repetition(concat); // Call the function under test
}

