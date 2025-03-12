// Answer 0

#[test]
fn test_get_swap_greed_enabled() {
    let config = Config::new().swap_greed(true);
    let result = config.get_swap_greed();
}

#[test]
fn test_get_swap_greed_disabled() {
    let config = Config::new().swap_greed(false);
    let result = config.get_swap_greed();
}

