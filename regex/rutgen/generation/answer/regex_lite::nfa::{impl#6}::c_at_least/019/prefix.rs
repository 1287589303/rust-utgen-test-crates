// Answer 0

#[test]
fn test_c_at_least_n_zero() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let pattern = ".*"; // Pattern that can match empty
    let hir = Hir::parse(config, pattern).unwrap();
    let compiler = Compiler { config, nfa: RefCell::new(NFA::default()) };
    
    let result = compiler.c_at_least(&hir, true, 0);
}

#[test]
fn test_c_at_least_n_zero_not_empty_match() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let pattern = "a"; // Pattern that cannot match empty
    let hir = Hir::parse(config, pattern).unwrap();
    let compiler = Compiler { config, nfa: RefCell::new(NFA::default()) };

    let result = compiler.c_at_least(&hir, true, 0);
}

