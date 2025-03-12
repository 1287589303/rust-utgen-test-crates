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
fn test_nfa_size_limit_one() {
    let config = Config::new().nfa_size_limit(Some(1));
}

#[test]
fn test_nfa_size_limit_fifty() {
    let config = Config::new().nfa_size_limit(Some(50));
}

#[test]
fn test_nfa_size_limit_one_hundred() {
    let config = Config::new().nfa_size_limit(Some(100));
}

#[test]
fn test_nfa_size_limit_four_hundred_thousand() {
    let config = Config::new().nfa_size_limit(Some(400_000));
}

#[test]
fn test_nfa_size_limit_five_hundred_thousand() {
    let config = Config::new().nfa_size_limit(Some(500_000));
}

#[test]
fn test_nfa_size_limit_one_million() {
    let config = Config::new().nfa_size_limit(Some(1_000_000));
}

