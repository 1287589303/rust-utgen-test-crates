// Answer 0

#[test]
fn test_c_at_least_case_hir_non_empty() {
    let config = Config { nest_limit: 0, flags: Flags::empty() };
    let compiler = Compiler::new(config.clone(), String::from("some pattern"));

    let hir = Hir::class(hir::Class::new(vec![('a', 'z')], false)); // is_match_empty() is true
    let greedy = true;
    let n = 1;

    let _result = compiler.c_at_least(&hir, greedy, n);
}

#[test]
fn test_c_at_least_case_hir_non_empty_greedy_false() {
    let config = Config { nest_limit: 0, flags: Flags::empty() };
    let compiler = Compiler::new(config.clone(), String::from("another pattern"));

    let hir = Hir::class(hir::Class::new(vec![('0', '9')], false)); // is_match_empty() is true
    let greedy = false;
    let n = 2;

    let _result = compiler.c_at_least(&hir, greedy, n);
}

#[test]
fn test_c_at_least_case_hir_non_empty_zero() {
    let config = Config { nest_limit: 0, flags: Flags::empty() };
    let compiler = Compiler::new(config.clone(), String::from("yetanother pattern"));

    let hir = Hir::class(hir::Class::new(vec![('A', 'Z')], false)); // is_match_empty() is true
    let greedy = true;
    let n = 3;

    let _result = compiler.c_at_least(&hir, greedy, n);
}

