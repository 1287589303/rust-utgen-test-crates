// Answer 0

#[test]
fn test_look_behind_none() {
    let config = Config::new();
    let result = config.look_behind(None);
}

#[test]
fn test_look_behind_minimum() {
    let config = Config::new();
    let result = config.look_behind(Some(0));
}

#[test]
fn test_look_behind_maximum() {
    let config = Config::new();
    let result = config.look_behind(Some(255));
}

