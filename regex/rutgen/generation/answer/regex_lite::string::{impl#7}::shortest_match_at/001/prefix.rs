// Answer 0

#[test]
fn test_shortest_match_at_valid_start_zero() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::default())), 
        pool: CachePool::default() 
    };
    let hay = "chew";
    let start = 0;
    let _ = re.shortest_match_at(hay, start);
}

#[test]
fn test_shortest_match_at_valid_start_middle() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::default())), 
        pool: CachePool::default() 
    };
    let hay = "schlew";
    let start = 1;
    let _ = re.shortest_match_at(hay, start);
}

#[test]
fn test_shortest_match_at_valid_start_last() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::default())), 
        pool: CachePool::default() 
    };
    let hay = "xchewy";
    let start = 1;
    let _ = re.shortest_match_at(hay, start);
}

#[test]
#[should_panic]
fn test_shortest_match_at_start_out_of_bounds() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::default())), 
        pool: CachePool::default() 
    };
    let hay = "sample";
    let start = hay.len(); // Invalid start
    let _ = re.shortest_match_at(hay, start);
}

