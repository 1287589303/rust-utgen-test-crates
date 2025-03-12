// Answer 0

#[test]
fn test_add_capture_name_duplicate() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "sample_pattern";
    let mut parser = Parser::new(config, pattern);
    
    let capture_name = "duplicate_name";
    let _ = parser.add_capture_name(capture_name);

    let result = parser.add_capture_name(capture_name);
}

#[test]
fn test_add_capture_name_duplicate_with_existing_name() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "sample_pattern";
    let mut parser = Parser::new(config, pattern);

    let capture_name_1 = "duplicate_name";
    let _ = parser.add_capture_name(capture_name_1);
    
    let capture_name_2 = "duplicate_name"; 
    let result = parser.add_capture_name(capture_name_2);
}

