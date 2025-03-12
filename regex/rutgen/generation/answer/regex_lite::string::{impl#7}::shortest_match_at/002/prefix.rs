// Answer 0

#[test]
fn test_shortest_match_at_out_of_bounds_start() {
    let re = Regex {
        pikevm: Arc::new(PikeVM::new(NFA::default())),
        pool: CachePool::default(),
    };
    let haystack = "example";
    let start = haystack.len() + 1; // start >= haystack.len() + 1
    let _result = re.shortest_match_at(haystack, start);
}

#[test]
fn test_shortest_match_at_exactly_out_of_bounds_start() {
    let re = Regex {
        pikevm: Arc::new(PikeVM::new(NFA::default())),
        pool: CachePool::default(),
    };
    let haystack = "test string";
    let start = haystack.len(); // start = haystack.len() + 1 will be next
    let _result = re.shortest_match_at(haystack, start);
}

