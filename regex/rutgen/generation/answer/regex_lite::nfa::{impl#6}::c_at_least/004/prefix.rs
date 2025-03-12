// Answer 0

#[test]
fn test_c_at_least_n_positive_hir_match_empty() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("test"));
    let hir = Hir::empty();

    let result = compiler.c_at_least(&hir, true, 1);
}

#[test]
fn test_c_at_least_n_positive_hir_match_empty_greedy_false() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("test"));
    let hir = Hir::empty();
    
    let result = compiler.c_at_least(&hir, false, 2);
}

#[test]
fn test_c_at_least_n_positive_hir_match_empty_with_states() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("test"));
    let hir = Hir::char('a');

    let result = compiler.c_at_least(&hir, true, 3);
}

