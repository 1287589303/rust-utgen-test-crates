// Answer 0

#[test]
fn test_specialize_start_states_true() {
    let config = Config::new().specialize_start_states(true);
    let _ = config;
}

#[test]
fn test_specialize_start_states_false() {
    let config = Config::new().specialize_start_states(false);
    let _ = config;
}

#[test]
fn test_specialize_start_states_none() {
    let mut config = Config::new();
    config.specialize_start_states(false);
    let _ = config;
}

