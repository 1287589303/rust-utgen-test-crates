// Answer 0

#[test]
fn test_c_at_least_n_zero() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "abc";
    let compiler = Compiler::new(config, pattern.to_string());
    let hir = Hir::parse(config, pattern).expect("Failed to parse Hir");

    let _ = compiler.c_at_least(&hir, true, 0);
}

#[test]
fn test_c_at_least_n_one() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a";
    let compiler = Compiler::new(config, pattern.to_string());
    let hir = Hir::parse(config, pattern).expect("Failed to parse Hir");

    let _ = compiler.c_at_least(&hir, false, 1);
}

#[test]
fn test_c_at_least_n_two() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "ab";
    let compiler = Compiler::new(config, pattern.to_string());
    let hir = Hir::parse(config, pattern).expect("Failed to parse Hir");
  
    let _ = compiler.c_at_least(&hir, true, 2);
}

#[test]
#[should_panic]
fn test_c_at_least_large_n() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "xyz";
    let compiler = Compiler::new(config, pattern.to_string());
    let hir = Hir::parse(config, pattern).expect("Failed to parse Hir");

    let _ = compiler.c_at_least(&hir, false, u32::MAX);
}

