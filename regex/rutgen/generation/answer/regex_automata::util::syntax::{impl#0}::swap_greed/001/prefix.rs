// Answer 0

#[test]
fn test_swap_greed_enable() {
    let config = Config::new().swap_greed(true);
}

#[test]
fn test_swap_greed_disable() {
    let config = Config::new().swap_greed(false);
}

#[test]
fn test_swap_greed_chainable() {
    let config = Config::new().swap_greed(true).swap_greed(false);
}

