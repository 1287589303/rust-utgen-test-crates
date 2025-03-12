// Answer 0

#[test]
fn test_c_at_least_n_1_hir_match_empty_true_greedy_true() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, "pattern".to_string());
    let hir = Hir::class(hir::Class::new(vec![('a', 'z')]));
    let result = compiler.c_at_least(&hir, true, 1);
}

#[test]
fn test_c_at_least_n_1_hir_match_empty_true_greedy_false() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, "pattern".to_string());
    let hir = Hir::class(hir::Class::new(vec![('a', 'z')]));
    let result = compiler.c_at_least(&hir, false, 1);
}

#[test]
fn test_c_at_least_n_greater_than_1_hir_match_empty_true_greedy_true() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, "pattern".to_string());
    let hir = Hir::class(hir::Class::new(vec![('a', 'z')]));
    let result = compiler.c_at_least(&hir, true, 2);
}

#[test]
fn test_c_at_least_n_greater_than_1_hir_match_empty_true_greedy_false() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, "pattern".to_string());
    let hir = Hir::class(hir::Class::new(vec![('a', 'z')]));
    let result = compiler.c_at_least(&hir, false, 2);
}

