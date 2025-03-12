// Answer 0

#[test]
fn test_get_nfa_size_limit_none() {
    let config = Config::new();
    assert_eq!(config.get_nfa_size_limit(), None);
}

#[test]
fn test_get_nfa_size_limit_zero() {
    let config = Config::new().nfa_size_limit(Some(0));
    assert_eq!(config.get_nfa_size_limit(), Some(0));
}

#[test]
fn test_get_nfa_size_limit_one() {
    let config = Config::new().nfa_size_limit(Some(1));
    assert_eq!(config.get_nfa_size_limit(), Some(1));
}

#[test]
fn test_get_nfa_size_limit_ten() {
    let config = Config::new().nfa_size_limit(Some(10));
    assert_eq!(config.get_nfa_size_limit(), Some(10));
}

#[test]
fn test_get_nfa_size_limit_hundred() {
    let config = Config::new().nfa_size_limit(Some(100));
    assert_eq!(config.get_nfa_size_limit(), Some(100));
}

#[test]
fn test_get_nfa_size_limit_max_usize() {
    let config = Config::new().nfa_size_limit(Some(usize::MAX));
    assert_eq!(config.get_nfa_size_limit(), Some(usize::MAX));
}

