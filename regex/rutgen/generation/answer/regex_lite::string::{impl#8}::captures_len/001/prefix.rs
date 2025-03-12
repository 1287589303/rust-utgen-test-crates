// Answer 0

#[test]
fn test_captures_len_no_groups() {
    let nfa = NFA::new(Config::default(), "foo".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_one_group() {
    let nfa = NFA::new(Config::default(), "(foo)".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_multiple_groups() {
    let nfa = NFA::new(Config::default(), r"(?<a>.(?<b>.))(.)(?:.)(?<c>.)".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_named_groups() {
    let nfa = NFA::new(Config::default(), r"(?<name>foo)".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_empty_match() {
    let nfa = NFA::new(Config::default(), r"[^\s\S]".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    let _ = regex.captures_len();
}

#[test]
fn test_captures_len_large_pattern() {
    let pattern = "a".repeat(1000); // A large pattern
    let nfa = NFA::new(Config::default(), pattern, &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    let _ = regex.captures_len();
}

