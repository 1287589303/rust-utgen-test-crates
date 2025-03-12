// Answer 0

#[test]
fn test_char_valid_position() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    
    let pattern = "abc";
    let mut parser = Parser::new(config, pattern);
    
    parser.pos.set(1); // Position set to a valid index
    let _result = parser.char();
}

#[test]
fn test_char_valid_utf8_character() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };

    let pattern = "你好"; // Valid UTF-8 string
    let mut parser = Parser::new(config, pattern);
    
    parser.pos.set(0); // Position set to the start
    let _result = parser.char();
}

#[test]
#[should_panic]
fn test_char_at_end_of_pattern() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };

    let pattern = "xyz";
    let mut parser = Parser::new(config, pattern);
    
    parser.pos.set(3); // Position set to end
    let _result = parser.char(); // Should panic here
}

#[test]
fn test_char_non_empty_pattern() {
    let config = Config {
        nest_limit: 5,
        flags: Flags::default(),
    };

    let pattern = "123"; 
    let mut parser = Parser::new(config, pattern);
    
    parser.pos.set(2); // Position to second last character
    let _result = parser.char();
}

#[test]
fn test_char_multiple_utf8_characters() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };

    let pattern = "abc中文测试"; // Mixed valid UTF-8 string
    let mut parser = Parser::new(config, pattern);
    
    parser.pos.set(4); // Position set to the first Chinese character
    let _result = parser.char();
}

