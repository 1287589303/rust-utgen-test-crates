// Answer 0

#[test]
fn test_too_many_states_case_1() {
    let given = 1;
    let limit = 0;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

#[test]
fn test_too_many_states_case_2() {
    let given = 5;
    let limit = 4;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

#[test]
fn test_too_many_states_case_3() {
    let given = 10;
    let limit = 9;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

#[test]
fn test_too_many_states_case_4() {
    let given = 100;
    let limit = 50;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

#[test]
fn test_too_many_states_case_5() {
    let given = 1000;
    let limit = 999;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

#[test]
fn test_too_many_states_case_6() {
    let given = 5000;
    let limit = 4999;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

#[test]
fn test_too_many_states_case_7() {
    let given = 10000;
    let limit = 9999;
    let error = BuildError::too_many_states(given);
    let _ = format!("{}", error);
}

