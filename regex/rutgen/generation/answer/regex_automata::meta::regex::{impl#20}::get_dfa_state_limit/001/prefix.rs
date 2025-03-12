// Answer 0

#[test]
fn test_get_dfa_state_limit_some_zero() {
    let config = Config::new().dfa_state_limit(Some(0));
    config.get_dfa_state_limit();
}

#[test]
fn test_get_dfa_state_limit_some_one() {
    let config = Config::new().dfa_state_limit(Some(1));
    config.get_dfa_state_limit();
}

#[test]
fn test_get_dfa_state_limit_some_thirty() {
    let config = Config::new().dfa_state_limit(Some(30));
    config.get_dfa_state_limit();
}

#[test]
fn test_get_dfa_state_limit_some_hundred() {
    let config = Config::new().dfa_state_limit(Some(100));
    config.get_dfa_state_limit();
}

#[test]
fn test_get_dfa_state_limit_none() {
    let config = Config::new();
    config.get_dfa_state_limit();
}

