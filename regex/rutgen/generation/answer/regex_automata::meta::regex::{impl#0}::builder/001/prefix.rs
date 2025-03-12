// Answer 0

#[test]
fn test_builder_basic() {
    let builder = Regex::builder();
    let result = builder.build(r"abc");
}

#[test]
fn test_builder_multi_line_mode() {
    let config = crate::util::syntax::Config::new().multi_line(true);
    let builder = Regex::builder();
    let result = builder.syntax(config).build(r"^foo$");
}

#[test]
fn test_builder_line_terminator() {
    let config = Regex::config().line_terminator(b'\x00');
    let builder = Regex::builder();
    let result = builder.configure(config).build(r"^foo$");
}

#[test]
fn test_builder_empty_pattern_array() {
    let builder = Regex::builder();
    let patterns: Vec<&str> = vec!["foo"];
    let result = builder.build_many(&patterns);
}

#[test]
fn test_builder_hir() {
    let hir_example = hir::Hir::literal("foo");
    let builder = Regex::builder();
    let result = builder.build_from_hir(&hir_example);
}

#[test]
fn test_builder_many_hir() {
    let hir_example1 = hir::Hir::literal("foo");
    let hir_example2 = hir::Hir::literal("bar");
    let hirs: Vec<&Hir> = vec![&hir_example1, &hir_example2];
    let builder = Regex::builder();
    let result = builder.build_many_from_hir(&hirs);
}

