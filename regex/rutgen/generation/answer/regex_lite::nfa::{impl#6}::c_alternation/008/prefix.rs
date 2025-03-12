// Answer 0

#[test]
fn test_c_alternation_with_two_valid_elements() {
    let config = Config { nest_limit: 5, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a|b"));

    let first_thompson_ref = ThompsonRef { start: 1, end: 2 };
    let second_thompson_ref = ThompsonRef { start: 3, end: 4 };
    
    let ite = vec![Ok(first_thompson_ref), Ok(second_thompson_ref)].into_iter();
    
    let _ = compiler.c_alternation(ite);
}

#[test]
fn test_c_alternation_with_three_valid_elements() {
    let config = Config { nest_limit: 5, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("a|b|c"));

    let first_thompson_ref = ThompsonRef { start: 1, end: 2 };
    let second_thompson_ref = ThompsonRef { start: 3, end: 4 };
    let third_thompson_ref = ThompsonRef { start: 5, end: 6 };

    let ite = vec![Ok(first_thompson_ref), Ok(second_thompson_ref), Ok(third_thompson_ref)].into_iter();

    let _ = compiler.c_alternation(ite);
}

#[test]
fn test_c_alternation_with_same_start_end() {
    let config = Config { nest_limit: 5, flags: Flags::empty() };
    let compiler = Compiler::new(config, String::from("x|y"));

    let thompson_ref = ThompsonRef { start: 1, end: 1 };

    let ite = vec![Ok(thompson_ref.clone()), Ok(thompson_ref)].into_iter();

    let _ = compiler.c_alternation(ite);
}

