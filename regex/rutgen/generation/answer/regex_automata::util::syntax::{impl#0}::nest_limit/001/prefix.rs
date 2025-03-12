// Answer 0

#[test]
fn test_nest_limit_zero() {
    let config = Config::new().nest_limit(0);
}

#[test]
fn test_nest_limit_one() {
    let config = Config::new().nest_limit(1);
}

#[test]
fn test_nest_limit_ten() {
    let config = Config::new().nest_limit(10);
}

#[test]
fn test_nest_limit_fifty() {
    let config = Config::new().nest_limit(50);
}

#[test]
fn test_nest_limit_one_hundred() {
    let config = Config::new().nest_limit(100);
}

#[test]
fn test_nest_limit_one_thousand() {
    let config = Config::new().nest_limit(1000);
}

