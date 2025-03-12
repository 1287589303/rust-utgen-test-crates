pub fn new() -> Config {
        // These defaults match the ones used in regex-syntax.
        Config {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            crlf: false,
            line_terminator: b'\n',
            swap_greed: false,
            ignore_whitespace: false,
            unicode: true,
            utf8: true,
            nest_limit: 250,
            octal: false,
        }
    }