// Answer 0

#[test]
fn test_build_invalid_regex_empty_pattern() {
    let builder = RegexBuilder::new("");
    let result = builder.build();
}

#[test]
fn test_build_invalid_regex_missing_closing_parenthesis() {
    let builder = RegexBuilder::new("(abc");
    let result = builder.build();
}

#[test]
fn test_build_invalid_regex_complex_nesting() {
    let builder = RegexBuilder::new("(((a)b)c)d)e");
    let result = builder.build();
}

#[test]
fn test_build_invalid_regex_invalid_character() {
    let builder = RegexBuilder::new("a$b");
    let result = builder.build();
}

#[test]
fn test_build_exceed_size_limit() {
    let mut builder = RegexBuilder::new("a".repeat(1025)); // Assuming 1024 is the size limit
    builder.nfa_config.size_limit = Some(1024);
    let result = builder.build();
}

#[test]
fn test_build_exceed_nest_limit() {
    let mut builder = RegexBuilder::new("((a(b(c(d(e(f(g(h(i(j(k(l(m(n(o(p(q(r(s(t(u(v(w(x(y(z())");
    builder.hir_config.nest_limit = 10; // Assuming 10 is the limit
    let result = builder.build();
}

