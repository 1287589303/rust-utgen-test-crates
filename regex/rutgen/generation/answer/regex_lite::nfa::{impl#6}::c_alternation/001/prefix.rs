// Answer 0

#[test]
fn test_c_alternation_valid_first_err_second() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a|b".to_string();
    let compiler = Compiler::new(config, pattern);

    let valid_thompson_ref = Ok(ThompsonRef { start: 1, end: 2 });
    let invalid_result: Result<ThompsonRef, Error> = Err(Error { msg: "error" });

    let iterator = vec![valid_thompson_ref, invalid_result].into_iter();

    let _ = compiler.c_alternation(iterator);
}

#[test]
fn test_c_alternation_valid_first_none_second() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a|b".to_string();
    let compiler = Compiler::new(config, pattern);

    let valid_thompson_ref = Ok(ThompsonRef { start: 1, end: 2 });
    let none_result: Result<ThompsonRef, Error> = Err(Error { msg: "error" });

    let iterator = vec![valid_thompson_ref, none_result].into_iter();

    let _ = compiler.c_alternation(iterator);
}

