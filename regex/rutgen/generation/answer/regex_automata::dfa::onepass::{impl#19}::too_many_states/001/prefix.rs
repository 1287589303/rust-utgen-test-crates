// Answer 0

#[test]
fn test_too_many_states_zero() {
    let limit = 0;
    let _result = BuildError::too_many_states(limit);
}

#[test]
fn test_too_many_states_one() {
    let limit = 1;
    let _result = BuildError::too_many_states(limit);
}

#[test]
fn test_too_many_states_thousand() {
    let limit = 1_000;
    let _result = BuildError::too_many_states(limit);
}

#[test]
fn test_too_many_states_ten_thousand() {
    let limit = 10_000;
    let _result = BuildError::too_many_states(limit);
}

#[test]
fn test_too_many_states_max_u64() {
    let limit = 18_446_744_073_709_551_615;
    let _result = BuildError::too_many_states(limit);
}

#[should_panic]
fn test_too_many_states_out_of_bounds() {
    let limit = 18_446_744_073_709_551_616;
    let _result = BuildError::too_many_states(limit);
}

