// Answer 0

#[test]
fn test_static_captures_len_single_character() {
    let nfa = NFA::new(Config::default(), "a".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_static_captures_len_single_capture_group() {
    let nfa = NFA::new(Config::default(), "(a)".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_static_captures_len_multiple_disjoint_groups() {
    let nfa = NFA::new(Config::default(), "(a)|(b)".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_static_captures_len_nested_groups() {
    let nfa = NFA::new(Config::default(), "(a(b))".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_static_captures_len_zero_capture_groups() {
    let nfa = NFA::new(Config::default(), "a".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_dynamic_captures_len_alternation() {
    let nfa = NFA::new(Config::default(), "(a)|b".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_dynamic_captures_len_optional_group() {
    let nfa = NFA::new(Config::default(), "a?(b)".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_dynamic_captures_len_kleene_star() {
    let nfa = NFA::new(Config::default(), "(b)*".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

#[test]
fn test_static_captures_len_plus_operator() {
    let nfa = NFA::new(Config::default(), "(b)+".to_string(), &Hir::default()).unwrap();
    let pikevm = PikeVM::new(nfa.clone());
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::default() };
    regex.static_captures_len();
}

