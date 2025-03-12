// Answer 0

#[test]
fn test_char_at_valid_start() {
    let pattern = "abcde";
    let parser = ParserI::new(Parser { /* initialization */ }, pattern);
    let result = parser.char_at(0);
}

#[test]
fn test_char_at_valid_middle() {
    let pattern = "abcde";
    let parser = ParserI::new(Parser { /* initialization */ }, pattern);
    let result = parser.char_at(2);
}

#[test]
fn test_char_at_valid_end() {
    let pattern = "abcde";
    let parser = ParserI::new(Parser { /* initialization */ }, pattern);
    let result = parser.char_at(4);
}

#[test]
#[should_panic]
fn test_char_at_out_of_bounds() {
    let pattern = "abcde";
    let parser = ParserI::new(Parser { /* initialization */ }, pattern);
    let result = parser.char_at(5);
}

