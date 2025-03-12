// Answer 0

#[test]
fn test_c_at_least_n_zero_then_empty() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(""));
    let hir = Hir::empty();
    let result = compiler.c_at_least(&hir, true, 0);
}

#[test]
fn test_c_at_least_n_one() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(""));
    let hir = Hir::char('a');
    let result = compiler.c_at_least(&hir, true, 1);
}

#[test]
fn test_c_at_least_n_two() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(""));
    let hir = Hir::char('b');
    let result = compiler.c_exactly(&hir, 1).unwrap();
    let result2 = compiler.c_at_least(&hir, true, 2);
    if let Err(_) = compiler.patch(result.end, result2.start) {
        // expected failure
    }
}

