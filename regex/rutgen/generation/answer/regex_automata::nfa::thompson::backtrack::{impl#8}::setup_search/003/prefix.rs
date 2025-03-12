// Answer 0

#[test]
fn test_setup_search_with_capacity_eq_max_capacity() {
    let mut visited = Visited { 
        bitset: vec![0; 2], 
        stride: 0 
    };
    let nfa_states_len = 1;
    let max_capacity = 8;
    let expected_capacity = max_capacity;

    let config = Config::default().visited_capacity(expected_capacity);
    let nfa = NFA::always_match();
    let re = BoundedBacktracker { config, nfa };

    let input = Input::new(b"abc").span(Span { start: 0, end: 3 });

    visited.setup_search(&re, &input).unwrap();
}

#[test]
fn test_setup_search_with_non_zero_length_bitset() {
    let mut visited = Visited { 
        bitset: vec![1; 2], 
        stride: 0 
    };
    let nfa_states_len = 1;
    let max_capacity = 8;
    let expected_capacity = max_capacity;

    let config = Config::default().visited_capacity(expected_capacity);
    let nfa = NFA::always_match();
    let re = BoundedBacktracker { config, nfa };

    let input = Input::new(b"abc").span(Span { start: 0, end: 3 });

    visited.setup_search(&re, &input).unwrap();
}

#[test]
fn test_setup_search_with_bitset_length_eq_needed_blocks() {
    let mut visited = Visited { 
        bitset: vec![0; 2], 
        stride: 0 
    };
    let nfa_states_len = 1;
    let max_capacity = 8;
    let expected_capacity = max_capacity;

    let config = Config::default().visited_capacity(expected_capacity);
    let nfa = NFA::always_match();
    let re = BoundedBacktracker { config, nfa };

    let input = Input::new(b"abc").span(Span { start: 0, end: 3 });

    visited.setup_search(&re, &input).unwrap();
}

