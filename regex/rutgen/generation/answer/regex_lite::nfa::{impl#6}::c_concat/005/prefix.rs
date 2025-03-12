// Answer 0

#[test]
fn test_c_concat_single_element_ok() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));

    let thompson_ref = ThompsonRef { start: 1, end: 2 };
    let iterator = std::iter::once(Ok(thompson_ref));

    let _result = compiler.c_concat(iterator);
}

#[test]
fn test_c_concat_single_element_ok_with_max_state_id() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));

    let thompson_ref = ThompsonRef { start: u32::MAX, end: u32::MAX };
    let iterator = std::iter::once(Ok(thompson_ref));

    let _result = compiler.c_concat(iterator);
}

#[test]
fn test_c_concat_single_element_ok_with_min_state_id() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));

    let thompson_ref = ThompsonRef { start: 0, end: 0 };
    let iterator = std::iter::once(Ok(thompson_ref));

    let _result = compiler.c_concat(iterator);
}

