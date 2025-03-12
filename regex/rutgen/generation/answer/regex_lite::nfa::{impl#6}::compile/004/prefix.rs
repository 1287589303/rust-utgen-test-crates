// Answer 0

#[test]
fn test_compile_valid_case_1() {
    let config = Config { nest_limit: 10, size_limit: None };
    let pattern = String::from("abc");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler::new(config, pattern);
    let _ = compiler.compile(&hir);
}

#[test]
fn test_compile_valid_case_2() {
    let config = Config { nest_limit: 20, size_limit: None };
    let pattern = String::from("a(bc)*d");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler::new(config, pattern);
    let _ = compiler.compile(&hir);
}

#[test]
fn test_compile_with_start_anchored() {
    let config = Config { nest_limit: 30, size_limit: None };
    let pattern = String::from("^xyz");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler::new(config, pattern);
    let _ = compiler.compile(&hir);
}

#[test]
fn test_compile_with_match_empty() {
    let config = Config { nest_limit: 40, size_limit: None };
    let pattern = String::from(".*");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler::new(config, pattern);
    let _ = compiler.compile(&hir);
}

#[test]
fn test_compile_with_static_explicit_captures() {
    let config = Config { nest_limit: 50, size_limit: None };
    let pattern = String::from("(abc)");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler::new(config, pattern);
    let _ = compiler.compile(&hir);
}

