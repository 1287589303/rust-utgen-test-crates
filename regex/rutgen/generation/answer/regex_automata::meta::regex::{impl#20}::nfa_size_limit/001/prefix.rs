// Answer 0

#[test]
fn test_nfa_size_limit_none() {
    let config = Config::new().nfa_size_limit(None);
}

#[test]
fn test_nfa_size_limit_zero() {
    let config = Config::new().nfa_size_limit(Some(0));
}

#[test]
fn test_nfa_size_limit_1024() {
    let config = Config::new().nfa_size_limit(Some(1024));
}

#[test]
fn test_nfa_size_limit_large() {
    let config = Config::new().nfa_size_limit(Some(1073741824));
}

#[test]
fn test_nfa_size_limit_20kb() {
    let config = Config::new().nfa_size_limit(Some(20 * (1 << 10)));
}

