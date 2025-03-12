// Answer 0

#[test]
fn test_c_at_least_n_zero() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a*"));
    let hir = Hir::empty(); // Considered to have match_empty=true for this test.
    let _ = compiler.c_at_least(&hir, true, 0);
}

#[test]
fn test_c_at_least_n_one() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("b"));
    let hir = Hir::char('b'); // Matches 'b', hence is_match_empty is false.
    let _ = compiler.c_at_least(&hir, false, 1);
}

#[test]
fn test_c_at_least_n_two() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("c"));
    let hir = Hir::char('c');
    let _ = compiler.c_at_least(&hir, true, 2);
}

#[test]
fn test_c_at_least_n_four() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("d"));
    let hir = Hir::char('d');
    let _ = compiler.c_at_least(&hir, false, 4);
}

#[test]
fn test_c_at_least_large_n() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("e"));
    let hir = Hir::char('e');
    let _ = compiler.c_at_least(&hir, true, 1000); // A large value to test size limits.
}

