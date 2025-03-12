// Answer 0

#[test]
fn test_dfa_size_limit_zero() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(0);
}

#[test]
fn test_dfa_size_limit_one() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(1);
}

#[test]
fn test_dfa_size_limit_ten() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(10);
}

#[test]
fn test_dfa_size_limit_kilobyte() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(1024);
}

#[test]
fn test_dfa_size_limit_sixty_five_kilobytes() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(65536);
}

#[test]
fn test_dfa_size_limit_u32_max() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(4294967295);
}

#[test]
fn test_dfa_size_limit_usize_max() {
    let mut builder = RegexBuilder::new("a");
    builder.dfa_size_limit(usize::MAX);
}

