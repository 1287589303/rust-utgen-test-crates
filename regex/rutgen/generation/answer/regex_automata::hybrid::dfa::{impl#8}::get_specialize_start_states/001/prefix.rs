// Answer 0

#[test]
fn test_get_specialize_start_states_none() {
    let config = Config::default();
    config.get_specialize_start_states();
}

#[test]
fn test_get_specialize_start_states_some_true() {
    let config = Config::new().specialize_start_states(true);
    config.get_specialize_start_states();
}

#[test]
fn test_get_specialize_start_states_some_false() {
    let config = Config::new().specialize_start_states(false);
    config.get_specialize_start_states();
}

