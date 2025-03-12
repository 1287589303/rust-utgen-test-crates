// Answer 0

#[test]
fn test_c_at_least_n_zero_is_match_empty_true() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let hir = Hir::empty(); // it's valid and is_match_empty will return true
    let result = compiler.c_at_least(&hir, true, 0);
}

#[test]
fn test_c_at_least_n_zero_is_match_empty_false() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a"));
    let hir = Hir::char('a'); // a valid resultant Hir, is_match_empty returns false
    let result = compiler.c_at_least(&hir, true, 0);
}

#[test]
fn test_c_at_least_n_one() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let hir = Hir::empty(); // valid and is_match_empty will return true
    let result = compiler.c_at_least(&hir, true, 1);
}

#[test]
fn test_c_at_least_n_greater_than_one_c_exactly_ok() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(".*"));
    let hir = Hir::char('a'); // valid and is_match_empty will return false
    let result = compiler.c_at_least(&hir, true, 2);
}

#[test]
fn test_c_at_least_n_greater_than_one_c_hir_err() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("invalid")); // assuming this input leads to an error in c(hir)
    let hir = Hir::char('b'); // causes an error in self.c(hir)?
    let result = compiler.c_at_least(&hir, false, 2);
}

