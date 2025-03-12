pub(super) fn new(config: Config, pattern: &'a str) -> Parser<'a> {
        Parser {
            config,
            pattern,
            depth: Cell::new(0),
            pos: Cell::new(0),
            char: Cell::new(pattern.chars().next()),
            capture_index: Cell::new(0),
            flags: RefCell::new(config.flags),
            capture_names: RefCell::new(vec![]),
        }
    }