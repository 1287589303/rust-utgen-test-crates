// Answer 0

#[test]
fn test_c_alternation_with_valid_results() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let pattern = String::from("a|b");
    let compiler = Compiler::new(config, pattern);

    let _ = compiler.add(State::Char { target: 1, ch: 'a' }).unwrap();
    let _ = compiler.add(State::Char { target: 2, ch: 'b' }).unwrap();
    
    let thompson_ref_a = ThompsonRef { start: 0, end: 1 };
    let thompson_ref_b = ThompsonRef { start: 1, end: 2 };
    
    let results = vec![
        Ok(thompson_ref_a),
        Ok(thompson_ref_b),
    ];
    
    let result_iter = results.into_iter();

    let _ = compiler.c_alternation(result_iter).unwrap();
}

#[test]
fn test_c_alternation_with_more_than_two_valid_results() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let pattern = String::from("a|b|c");
    let compiler = Compiler::new(config, pattern);

    let _ = compiler.add(State::Char { target: 1, ch: 'a' }).unwrap();
    let _ = compiler.add(State::Char { target: 2, ch: 'b' }).unwrap();
    let _ = compiler.add(State::Char { target: 3, ch: 'c' }).unwrap();

    let thompson_ref_a = ThompsonRef { start: 0, end: 1 };
    let thompson_ref_b = ThompsonRef { start: 1, end: 2 };
    let thompson_ref_c = ThompsonRef { start: 2, end: 3 };
    
    let results = vec![
        Ok(thompson_ref_a),
        Ok(thompson_ref_b),
        Ok(thompson_ref_c),
    ];
    
    let result_iter = results.into_iter();

    let _ = compiler.c_alternation(result_iter).unwrap();
}

#[test]
fn test_c_alternation_with_valid_results_and_no_extra() {
    let config = Config { nest_limit: 10, flags: Flags::empty() };
    let pattern = String::from("x|y");
    let compiler = Compiler::new(config, pattern);

    let _ = compiler.add(State::Char { target: 1, ch: 'x' }).unwrap();
    let _ = compiler.add(State::Char { target: 2, ch: 'y' }).unwrap();

    let thompson_ref_x = ThompsonRef { start: 0, end: 1 };
    let thompson_ref_y = ThompsonRef { start: 1, end: 2 };
    
    let results = vec![
        Ok(thompson_ref_x),
        Ok(thompson_ref_y),
    ];
    
    let result_iter = results.into_iter();

    let _ = compiler.c_alternation(result_iter).unwrap();
}

