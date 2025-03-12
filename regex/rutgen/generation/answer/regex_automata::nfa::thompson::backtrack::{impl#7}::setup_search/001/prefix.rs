// Answer 0

#[test]
fn test_setup_search_empty_input() {
    let re = BoundedBacktracker { 
        config: Config::new(), 
        nfa: NFA::new(),
    };
    let input = Input {
        haystack: &[],
        span: Span::new(0, 0),
        anchored: Anchored::Yes,
        earliest: true,
    };
    let mut cache = Cache::new(&re);
    let _ = cache.setup_search(&re, &input);
}

#[test]
fn test_setup_search_max_capacity_input() {
    let re = BoundedBacktracker { 
        config: Config::new_with_visited_capacity(1), 
        nfa: NFA::new_with_states(1),
    };
    let input_haystack = vec![0; Visited::BLOCK_SIZE]; // setting to the maximum allowed size
    let input = Input {
        haystack: &input_haystack,
        span: Span::new(0, input_haystack.len()),
        anchored: Anchored::Yes,
        earliest: true,
    };
    let mut cache = Cache::new(&re);
    let _ = cache.setup_search(&re, &input);
}

#[test]
#[should_panic]
fn test_setup_search_exceeds_capacity() {
    let re = BoundedBacktracker { 
        config: Config::new_with_visited_capacity(1), 
        nfa: NFA::new_with_states(3),
    };
    let input_haystack = vec![0; Visited::BLOCK_SIZE + 1]; // exceeding maximum allowed size
    let input = Input {
        haystack: &input_haystack,
        span: Span::new(0, input_haystack.len()),
        anchored: Anchored::Yes,
        earliest: true,
    };
    let mut cache = Cache::new(&re);
    let _ = cache.setup_search(&re, &input);
}

#[test]
fn test_setup_search_non_empty_input() {
    let re = BoundedBacktracker { 
        config: Config::new(), 
        nfa: NFA::new_with_states(2),
    };
    let input_haystack = vec![1, 2, 3, 4, 5]; // valid length
    let input = Input {
        haystack: &input_haystack,
        span: Span::new(0, input_haystack.len()),
        anchored: Anchored::Yes,
        earliest: true,
    };
    let mut cache = Cache::new(&re);
    let _ = cache.setup_search(&re, &input);
}

