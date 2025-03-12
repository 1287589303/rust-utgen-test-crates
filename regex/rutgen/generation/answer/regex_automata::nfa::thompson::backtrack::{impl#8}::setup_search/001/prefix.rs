// Answer 0

#[test]
fn test_setup_search_with_overflow() {
    let mut visited = Visited {
        bitset: Vec::new(),
        stride: 0,
    };

    let re = BoundedBacktracker {
        config: Config::default().visited_capacity(1), // Set visited capacity to a small number
        nfa: NFA::never_match(), // Use a dummy NFA
    };

    let haystack = b"test haystack";
    let span = Span { start: 0, end: haystack.len() };
    let input = Input::new(haystack).span(span);

    visited.setup_search(&re, &input).unwrap_err(); // Expect an error due to overflow on capacity
}

#[test]
fn test_setup_search_edge_case() {
    let mut visited = Visited {
        bitset: Vec::new(),
        stride: usize::MAX, // Set stride to maximum value
    };

    let re = BoundedBacktracker {
        config: Config::default().visited_capacity(1), // Set visited capacity to a small number
        nfa: NFA::never_match(), // Use a dummy NFA 
    };

    let haystack = b"test haystack long enough";
    let span = Span { start: 0, end: haystack.len() };
    let input = Input::new(haystack).span(span);

    visited.setup_search(&re, &input).unwrap_err(); // Expect an error due to overflow on capacity
}

