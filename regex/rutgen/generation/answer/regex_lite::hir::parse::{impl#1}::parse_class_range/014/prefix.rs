// Answer 0

#[test]
fn test_parse_class_range_invalid_after_dash() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a-b"; // Pattern that represents a character class range
    let mut union: Vec<hir::ClassRange> = Vec::new();
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('-')), // Set char to '-'
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    // Simulate parser state for preconditions
    parser.bump_space = || { /* simulate space bump */ };
    parser.is_done = || false; // Not done
    parser.bump_and_bump_space = || false; // Should return false
    
    // Invoke the function under test and omit assertions
    let result = parser.parse_class_range(&mut union);
}

