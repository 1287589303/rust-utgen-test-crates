// Answer 0

#[test]
fn test_capture_locations_zero_captures() {
    let nfa = NFA::new(Config::default(), String::from(""), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let locs = regex.capture_locations();
}

#[test]
fn test_capture_locations_one_capture() {
    let nfa = NFA::new(Config::default(), String::from(r"()"), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let locs = regex.capture_locations();
}

#[test]
fn test_capture_locations_two_captures() {
    let nfa = NFA::new(Config::default(), String::from(r"(.)(.)"), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let locs = regex.capture_locations();
}

#[test]
fn test_capture_locations_three_captures() {
    let nfa = NFA::new(Config::default(), String::from(r"(.)(.)(\w+)"), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let locs = regex.capture_locations();
}

#[test]
fn test_capture_locations_ten_captures() {
    let nfa = NFA::new(Config::default(), String::from(r"(.)(.)(.)(.)(.)(.)(.)(.)(.)(.)"), &Hir::new()).unwrap();
    let pikevm = PikeVM::new(nfa);
    let regex = Regex { pikevm: Arc::new(pikevm), pool: CachePool::new() };
    let locs = regex.capture_locations();
}

