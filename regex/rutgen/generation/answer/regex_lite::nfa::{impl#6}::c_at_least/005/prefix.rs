// Answer 0

#[test]
fn test_c_at_least_with_non_zero_n_when_hir_can_match_empty_and_errors_on_add() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = String::from("a*");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler { config, nfa: RefCell::new(NFA::new(pattern.clone())) };

    let result = compiler.c_at_least(&hir, true, 1);
    let _ = result; // Ignore the result to complete the test.
}

#[test]
fn test_c_at_least_with_non_zero_n_hir_can_match_empty_and_after_compile() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = String::from("a*");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler { config, nfa: RefCell::new(NFA::new(pattern.clone())) };

    let _ = compiler.c(&hir).unwrap(); // Ensure this is Ok/Some
    let result = compiler.c_at_least(&hir, true, 2);
    let _ = result; // Ignore the result to complete the test.
}

#[test]
fn test_c_at_least_with_hir_making_empty_string() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = String::from("a?");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler { config, nfa: RefCell::new(NFA::new(pattern.clone())) };

    let result = compiler.c_at_least(&hir, false, 1);
    let _ = result; // Ignore the result to complete the test.
}

#[test]
fn test_c_at_least_with_max_n_for_non_empty_hir() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = String::from("b+");
    let hir = Hir::parse(config.clone(), &pattern).unwrap();
    let compiler = Compiler { config, nfa: RefCell::new(NFA::new(pattern.clone())) };

    let _ = compiler.c(&hir).unwrap(); // Ensure this is Ok/Some
    let result = compiler.c_at_least(&hir, true, u32::MAX);
    let _ = result; // Ignore the result to complete the test.
}

