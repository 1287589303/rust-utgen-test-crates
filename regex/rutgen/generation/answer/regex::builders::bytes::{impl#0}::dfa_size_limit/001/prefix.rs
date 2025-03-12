// Answer 0

#[test]
fn test_dfa_size_limit_zero() {
    let mut regex_builder = RegexBuilder::new(".*");
    regex_builder.dfa_size_limit(0);
}

#[test]
fn test_dfa_size_limit_one() {
    let mut regex_builder = RegexBuilder::new(".*");
    regex_builder.dfa_size_limit(1);
}

#[test]
fn test_dfa_size_limit_max() {
    let mut regex_builder = RegexBuilder::new(".*");
    regex_builder.dfa_size_limit(usize::MAX);
}

