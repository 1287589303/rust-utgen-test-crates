// Answer 0

#[test]
fn test_get_specialize_start_states_none() {
    let config = Config {
        specialize_start_states: None,
        ..Config::default()
    };
    let _ = config.get_specialize_start_states();
}

#[test]
fn test_get_specialize_start_states_some_true() {
    let config = Config {
        specialize_start_states: Some(true),
        ..Config::default()
    };
    let _ = config.get_specialize_start_states();
}

#[test]
fn test_get_specialize_start_states_some_false() {
    let config = Config {
        specialize_start_states: Some(false),
        ..Config::default()
    };
    let _ = config.get_specialize_start_states();
}

