// Answer 0

#[test]
fn test_capture_names_with_named_and_unnamed_groups() {
    let nfa = NFA::new(Config::default(), r"(?<a>.(?<b>.))(.)(?:.)(?<c>.)".to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let mut names = regex.capture_names();
    let _ = names.next();
    let _ = names.next();
    let _ = names.next();
    let _ = names.next();
    let _ = names.next();
    let _ = names.next();
}

#[test]
fn test_capture_names_with_empty_pattern() {
    let nfa = NFA::new(Config::default(), "".to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let mut names = regex.capture_names();
    let _ = names.next();
    let _ = names.next();
}

#[test]
fn test_capture_names_with_non_matching_pattern() {
    let nfa = NFA::new(Config::default(), r"[^\s\S]".to_string(), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let mut names = regex.capture_names();
    let _ = names.next();
    let _ = names.next();
}

