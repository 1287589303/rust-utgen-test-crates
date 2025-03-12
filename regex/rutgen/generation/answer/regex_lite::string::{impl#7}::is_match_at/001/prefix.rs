// Answer 0

#[test]
fn test_is_match_at_start_zero() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::new())), 
        pool: CachePool::new() 
    };
    let hay = "test";
    re.is_match_at(hay, 0);
}

#[test]
fn test_is_match_at_start_middle() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::new())), 
        pool: CachePool::new() 
    };
    let hay = "testing";
    re.is_match_at(hay, 3);
}

#[test]
fn test_is_match_at_start_end() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::new())), 
        pool: CachePool::new() 
    };
    let hay = "boundary";
    re.is_match_at(hay, hay.len());
}

#[should_panic]
#[test]
fn test_is_match_at_start_out_of_bounds() {
    let re = Regex { 
        pikevm: Arc::new(PikeVM::new(NFA::new())), 
        pool: CachePool::new() 
    };
    let hay = "panic";
    re.is_match_at(hay, hay.len() + 1);
}

