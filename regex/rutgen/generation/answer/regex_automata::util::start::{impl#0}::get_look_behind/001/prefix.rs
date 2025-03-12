// Answer 0

#[test]
fn test_get_look_behind_none() {
    let config = Config::new().look_behind(None);
    let result = config.get_look_behind();
}

#[test]
fn test_get_look_behind_some_zero() {
    let config = Config::new().look_behind(Some(0));
    let result = config.get_look_behind();
}

#[test]
fn test_get_look_behind_some_max() {
    let config = Config::new().look_behind(Some(255));
    let result = config.get_look_behind();
}

