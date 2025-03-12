// Answer 0

#[test]
fn test_build_simple_pattern() {
    let compiler = Compiler::new();
    let nfa = compiler.build("a").unwrap();
}

#[test]
fn test_build_complex_pattern() {
    let compiler = Compiler::new();
    let nfa = compiler.build("a|b|c").unwrap();
}

#[test]
fn test_build_empty_pattern() {
    let compiler = Compiler::new();
    let nfa = compiler.build("").unwrap();
}

#[test]
fn test_build_invalid_pattern() {
    let compiler = Compiler::new();
    let result = compiler.build("[");
    assert!(result.is_err());
}

#[test]
fn test_build_pattern_exceeding_size_limits() {
    let mut compiler = Compiler::new();
    let config = Config { nfa_size_limit: Some(1), ..Default::default() };
    compiler.configure(config);
    let result = compiler.build("a{100}");
    assert!(result.is_err());
}

#[test]
fn test_build_pattern_generating_excessive_states() {
    let compiler = Compiler::new();
    let result = compiler.build("(a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z){1000}");
    assert!(result.is_err());
}

#[test]
fn test_build_with_utf8_enabled() {
    let mut compiler = Compiler::new();
    let config = Config { utf8: Some(true), ..Default::default() };
    compiler.configure(config);
    let nfa = compiler.build(r"\w").unwrap();
}

#[test]
fn test_build_with_reverse_enabled() {
    let mut compiler = Compiler::new();
    let config = Config { reverse: Some(true), ..Default::default() };
    compiler.configure(config);
    let nfa = compiler.build("abc").unwrap();
}

