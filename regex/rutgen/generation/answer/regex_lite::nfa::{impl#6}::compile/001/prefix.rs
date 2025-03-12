// Answer 0

#[test]
fn compile_empty_pattern_fail() {
    let config = Config { nest_limit: 0, flags: Flags::default() };
    let pattern = "";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

#[test]
fn compile_single_character_pattern_fail() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "a";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

#[test]
fn compile_nested_capture_limit_fail() {
    let config = Config { nest_limit: 0, flags: Flags::default() };
    let pattern = "(a(b))";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

#[test]
fn compile_no_capture_fail() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "abc";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

#[test]
fn compile_multiple_captures_fail() {
    let config = Config { nest_limit: 2, flags: Flags::default() };
    let pattern = "(a)(b)";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

#[test]
fn compile_with_start_anchored_fail() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = "^abc";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

#[test]
fn compile_with_match_empty_fail() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let pattern = ".*";
    let hir = Hir::parse(config.clone(), pattern).unwrap();
    let compiler = Compiler::new(config, pattern.to_string());
    let result = compiler.compile(&hir);
}

