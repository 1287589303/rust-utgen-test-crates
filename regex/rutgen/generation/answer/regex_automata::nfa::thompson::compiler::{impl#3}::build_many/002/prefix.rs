// Answer 0

#[test]
fn test_build_many_valid_patterns() {
    let compiler = Compiler::new();
    let patterns = vec![
        r"(?-u)\s",   // valid regex
        r"(?-u)\w",   // valid regex
        r"(?-u)[a-zA-Z]", // valid regex
    ];
    let _ = compiler.build_many(&patterns);
}

#[test]
fn test_build_many_empty_patterns() {
    let compiler = Compiler::new();
    let patterns: Vec<&str> = vec![]; // empty slice
    let _ = compiler.build_many(&patterns);
}

#[test]
fn test_build_many_invalid_pattern() {
    let compiler = Compiler::new();
    let patterns = vec![
        r"(?-u)\s", // valid regex
        r"(?-u)[a-zA-Z]", // valid regex
        r"[*]", // invalid regex
    ];
    let _ = compiler.build_many(&patterns);
}

