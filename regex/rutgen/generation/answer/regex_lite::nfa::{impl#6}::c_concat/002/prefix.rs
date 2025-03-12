// Answer 0

#[test]
fn test_c_concat_with_success_then_failure() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a|b"));

    let ok_result = ThompsonRef { start: 1, end: 2 };
    let err_result = Err(Error { msg: "Test Error" });

    let iterator = vec![Ok(ok_result), err_result].into_iter();

    let _ = compiler.c_concat(iterator);
}

#[test]
fn test_c_concat_multiple_successes() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("x|y|z"));

    let first_ok = ThompsonRef { start: 3, end: 4 };
    let second_ok = ThompsonRef { start: 5, end: 6 };

    let iterator = vec![Ok(first_ok), Ok(second_ok)].into_iter();

    let _ = compiler.c_concat(iterator);
}

#[test]
fn test_c_concat_empty_iterator() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from(""));

    let iterator: Vec<Result<ThompsonRef, Error>> = vec![];

    let _ = compiler.c_concat(iterator.into_iter());
}

