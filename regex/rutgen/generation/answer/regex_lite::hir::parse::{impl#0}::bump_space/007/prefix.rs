// Answer 0

#[test]
fn test_bump_space_no_ignore_whitespace() {
    let config = Config { 
        nest_limit: 10, 
        flags: Flags { ignore_whitespace: false, ..Default::default() } 
    };
    let parser = Parser::new(config, "   a   b   # comment \n   c  d");
    parser.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_only_whitespace() {
    let config = Config { 
        nest_limit: 10, 
        flags: Flags { ignore_whitespace: false, ..Default::default() } 
    };
    let parser = Parser::new(config, "       ");
    parser.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_only_comments() {
    let config = Config { 
        nest_limit: 10, 
        flags: Flags { ignore_whitespace: false, ..Default::default() } 
    };
    let parser = Parser::new(config, "# this is a comment\n# another comment");
    parser.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_mixed() {
    let config = Config { 
        nest_limit: 10, 
        flags: Flags { ignore_whitespace: false, ..Default::default() } 
    };
    let parser = Parser::new(config, "   # comment\na# comment\nb   c");
    parser.bump_space();
}

#[test]
fn test_bump_space_no_ignore_whitespace_edge_case() {
    let config = Config { 
        nest_limit: 10, 
        flags: Flags { ignore_whitespace: false, ..Default::default() } 
    };
    let parser = Parser::new(config, "");
    parser.bump_space();
}

