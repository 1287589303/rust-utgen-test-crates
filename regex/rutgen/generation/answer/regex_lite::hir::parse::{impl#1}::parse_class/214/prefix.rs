// Answer 0

#[test]
fn test_parse_class_intersection_unsupported() {
    let parser = {
        let config = Config {
            size_limit: None,
        };

        let pattern = "[&--]";
        let depth = Cell::new(0);
        let pos = Cell::new(0);
        let char = Cell::new(Some('['));
        let capture_index = Cell::new(0);
        let flags = RefCell::new(Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        });
        let capture_names = RefCell::new(vec![]);
        Parser {
            config,
            pattern,
            depth,
            pos,
            char,
            capture_index,
            flags,
            capture_names,
        }
    };
    parser.bump_and_bump_space();
    parser.char.set(Some('-'));
    let union = vec!['-', '-'];
    parser.bump_space();
    parser.char.set(Some('&'));
    parser.peek = Some('&');

    let _ = parser.parse_class();
} 

#[test]
fn test_parse_class_intersection_unsupported_negated() {
    let parser = {
        let config = Config {
            size_limit: None,
        };

        let pattern = "[^&--]";
        let depth = Cell::new(0);
        let pos = Cell::new(0);
        let char = Cell::new(Some('['));
        let capture_index = Cell::new(0);
        let flags = RefCell::new(Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        });
        let capture_names = RefCell::new(vec![]);
        Parser {
            config,
            pattern,
            depth,
            pos,
            char,
            capture_index,
            flags,
            capture_names,
        }
    };
    parser.bump_and_bump_space();
    parser.char.set(Some('^'));
    parser.bump_and_bump_space();
    let union = vec!['-', '-'];
    parser.bump_space();
    parser.char.set(Some('&'));
    parser.peek = Some('&');

    let _ = parser.parse_class();
}

