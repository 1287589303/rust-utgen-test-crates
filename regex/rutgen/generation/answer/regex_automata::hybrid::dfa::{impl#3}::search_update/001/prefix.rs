// Answer 0

#[test]
fn test_search_update_valid() {
    let mut cache = Cache::new(&DFA::new());
    cache.search_start(0);
    cache.search_update(0);
}

#[test]
#[should_panic]
fn test_search_update_no_search_started() {
    let mut cache = Cache::new(&DFA::new());
    cache.search_update(1);
}

#[test]
fn test_search_update_boundary_case_min() {
    let mut cache = Cache::new(&DFA::new());
    cache.search_start(0);
    cache.search_update(0);
}

#[test]
fn test_search_update_boundary_case_max() {
    let mut cache = Cache::new(&DFA::new());
    cache.search_start(10);
    cache.search_update(10);
}

#[test]
fn test_search_update_less_than_start() {
    let mut cache = Cache::new(&DFA::new());
    cache.search_start(5);
    cache.search_update(4);
}

