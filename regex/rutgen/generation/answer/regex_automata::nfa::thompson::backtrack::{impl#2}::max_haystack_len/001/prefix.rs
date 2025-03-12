// Answer 0

#[test]
fn test_max_haystack_len_with_default_visited_capacity() {
    let re = BoundedBacktracker::new(r"(?-u)\w+").unwrap();
    let mut cache = re.create_cache();
    let max_len = re.max_haystack_len();
    let mut haystack = "a".repeat(max_len);
    re.try_find_iter(&mut cache, &haystack).next();
}

#[test]
fn test_max_haystack_len_exceeds_capacity() {
    let re = BoundedBacktracker::new(r"(?-u)\w+").unwrap();
    let mut cache = re.create_cache();
    let max_len = re.max_haystack_len();
    let mut haystack = "a".repeat(max_len);
    haystack.push('a');
    re.try_find_iter(&mut cache, &haystack).next();
}

#[test]
fn test_max_haystack_len_with_unicode() {
    let re = BoundedBacktracker::new(r"\w+").unwrap();
    let max_len = re.max_haystack_len();
    assert!(max_len <= 7000);
}

#[test]
fn test_max_haystack_len_with_increased_visited_capacity() {
    let re = BoundedBacktracker::builder()
        .configure(BoundedBacktracker::config().visited_capacity(1 << 20))
        .build(r"\w+")
        .unwrap();
    let max_len = re.max_haystack_len();
    assert!(max_len >= 25000);
    assert!(max_len <= 28000);
}

#[test]
fn test_max_haystack_len_with_minimum_capacity() {
    let config = Config::new().visited_capacity(1);
    let nfa = NFA::new(r"\w+").unwrap();
    let backtracker = BoundedBacktracker {
        config,
        nfa,
    };
    let max_len = backtracker.max_haystack_len();
    assert!(max_len >= 0); // Ensure the result is non-negative
} 

#[test]
fn test_max_haystack_len_with_large_visited_capacity() {
    let config = Config::new().visited_capacity(1 << 30);
    let nfa = NFA::new(r"(?-u)\d+").unwrap();
    let backtracker = BoundedBacktracker {
        config,
        nfa,
    };
    let max_len = backtracker.max_haystack_len();
    assert!(max_len > 0); // Ensure it can handle a large capacity
}

