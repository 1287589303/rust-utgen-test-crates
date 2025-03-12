// Answer 0

#[test]
fn test_octal_enabled() {
    let patterns = vec![r"\141"];
    let regex_set_builder = RegexSetBuilder::new(patterns);
    let _ = regex_set_builder.octal(true);
}

#[test]
fn test_octal_disabled() {
    let patterns = vec![r"\141"];
    let regex_set_builder = RegexSetBuilder::new(patterns);
    let _ = regex_set_builder.octal(false);
}

