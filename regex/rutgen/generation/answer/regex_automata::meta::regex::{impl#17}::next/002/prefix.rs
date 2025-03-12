// Answer 0

#[test]
fn test_next_limit_zero() {
    let haystack: &[u8] = b"test haystack";
    let span = Span { start: 0, end: 10 };
    let input = Input::new(&haystack).span(span);
    
    let finder = FindMatches {
        it: Searcher::new(input),
    };
    
    let splits = Split {
        finder,
        last: 5,
    };
    
    let mut split_n = SplitN {
        splits,
        limit: 0,
    };

    let _result = split_n.next();
}

#[test]
fn test_next_limit_greater_than_zero() {
    let haystack: &[u8] = b"test haystack";
    let span = Span { start: 0, end: 10 };
    let input = Input::new(&haystack).span(span);

    let finder = FindMatches {
        it: Searcher::new(input),
    };

    let splits = Split {
        finder,
        last: 7,
    };

    let mut split_n = SplitN {
        splits,
        limit: 1,
    };

    let _result = split_n.next();
}

