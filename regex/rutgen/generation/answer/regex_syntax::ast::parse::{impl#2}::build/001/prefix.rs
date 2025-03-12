// Answer 0

#[test]
fn test_build_parser_default() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 0,
        octal: false,
        empty_min_range: false,
    };
    let _parser = builder.build();
}

#[test]
fn test_build_parser_with_ignore_whitespace() {
    let builder = ParserBuilder {
        ignore_whitespace: true,
        nest_limit: 1,
        octal: false,
        empty_min_range: false,
    };
    let _parser = builder.build();
}

#[test]
fn test_build_parser_with_nest_limit() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 100,
        octal: false,
        empty_min_range: false,
    };
    let _parser = builder.build();
}

#[test]
fn test_build_parser_with_octal() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 0,
        octal: true,
        empty_min_range: false,
    };
    let _parser = builder.build();
}

#[test]
fn test_build_parser_with_empty_min_range() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 0,
        octal: false,
        empty_min_range: true,
    };
    let _parser = builder.build();
}

#[test]
fn test_build_parser_with_all_true() {
    let builder = ParserBuilder {
        ignore_whitespace: true,
        nest_limit: 100,
        octal: true,
        empty_min_range: true,
    };
    let _parser = builder.build();
}

#[test]
fn test_build_parser_with_all_false() {
    let builder = ParserBuilder {
        ignore_whitespace: false,
        nest_limit: 0,
        octal: false,
        empty_min_range: false,
    };
    let _parser = builder.build();
}

