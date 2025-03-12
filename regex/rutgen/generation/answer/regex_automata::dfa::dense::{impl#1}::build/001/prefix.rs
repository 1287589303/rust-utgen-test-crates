// Answer 0

#[test]
fn test_build_valid_regex() {
    let builder = Builder::new();
    let valid_pattern = "abc";
    let _result = builder.build(valid_pattern);
}

#[test]
fn test_build_valid_regex_wildcard() {
    let builder = Builder::new();
    let valid_pattern = ".*";
    let _result = builder.build(valid_pattern);
}

#[test]
fn test_build_valid_regex_start_anchor() {
    let builder = Builder::new();
    let valid_pattern = "^[a-zA-Z]+$";
    let _result = builder.build(valid_pattern);
}

#[test]
#[should_panic]
fn test_build_empty_regex() {
    let builder = Builder::new();
    let empty_pattern = "";
    let _result = builder.build(empty_pattern);
}

#[test]
#[should_panic]
fn test_build_invalid_regex_open_paren() {
    let builder = Builder::new();
    let invalid_pattern = "(";
    let _result = builder.build(invalid_pattern);
}

#[test]
#[should_panic]
fn test_build_invalid_regex_unmatched_alternation() {
    let builder = Builder::new();
    let invalid_pattern = "(a|b";
    let _result = builder.build(invalid_pattern);
}

#[test]
#[should_panic]
fn test_build_invalid_regex_character_class() {
    let builder = Builder::new();
    let invalid_pattern = "[a-z";
    let _result = builder.build(invalid_pattern);
}

