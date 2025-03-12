// Answer 0

#[test]
fn test_ignore_whitespace_true() {
    let pattern = r"
        \b
        (?<first>[A-Z]\w*)
        \s+
        (?<last>[A-Z]\w*)
        \b
    ";
    let mut builder = RegexBuilder::new(pattern);
    let returned_builder = builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_false() {
    let pattern = r"
        \b
        (?<first>[A-Z]\w*)
        \s+
        (?<last>[A-Z]\w*)
        \b
    ";
    let mut builder = RegexBuilder::new(pattern);
    let returned_builder = builder.ignore_whitespace(false);
}

#[test]
fn test_ignore_whitespace_single_space() {
    let pattern = r"
        \b
        (?<first>[A-Z]\w*) 
        (?<last>[A-Z]\w*)
        \b
    ";
    let mut builder = RegexBuilder::new(pattern);
    let returned_builder = builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_multiple_spaces() {
    let pattern = r"
        \b
        (?<first>[A-Z]\w*)     
        (?<last>[A-Z]\w*)
        \b
    ";
    let mut builder = RegexBuilder::new(pattern);
    let returned_builder = builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_special_characters() {
    let pattern = r"
        \b
        (?<first>[A-Z]\w*) # First name
        \s+
        (?<last>[A-Z]\w*)  # Last name
        \b
    ";
    let mut builder = RegexBuilder::new(pattern);
    let returned_builder = builder.ignore_whitespace(true);
}

