// Answer 0

#[test]
fn test_into_matches_iter_empty_input() {
    let haystack: &[u8] = b"";
    let input = Input {
        haystack,
        span: Span::new(0, 0),
        anchored: Anchored::False,
        earliest: false,
    };
    
    let searcher = Searcher::new(input);
    let _iter = searcher.into_matches_iter(|_| Ok(None));
}

#[test]
fn test_into_matches_iter_single_byte() {
    let haystack: &[u8] = b"a";
    let input = Input {
        haystack,
        span: Span::new(0, 1),
        anchored: Anchored::False,
        earliest: false,
    };

    let searcher = Searcher::new(input);
    let _iter = searcher.into_matches_iter(|_| Ok(None));
}

#[test]
fn test_into_matches_iter_small_haystack_no_matches() {
    let haystack: &[u8] = b"abc";
    let input = Input {
        haystack,
        span: Span::new(0, 3),
        anchored: Anchored::False,
        earliest: false,
    };

    let searcher = Searcher::new(input);
    let _iter = searcher.into_matches_iter(|_| Ok(None));
}

#[test]
fn test_into_matches_iter_small_haystack_with_matches() {
    let haystack: &[u8] = b"aaaa";
    let input = Input {
        haystack,
        span: Span::new(0, 4),
        anchored: Anchored::False,
        earliest: false,
    };

    let searcher = Searcher::new(input);
    let _iter = searcher.into_matches_iter(|_| Ok(Some(Match::must(0, 0..1))));
}

#[test]
fn test_into_matches_iter_large_haystack() {
    let haystack: Vec<u8> = vec![b'a'; 10000];
    let input = Input {
        haystack: &haystack,
        span: Span::new(0, 10000),
        anchored: Anchored::False,
        earliest: false,
    };

    let searcher = Searcher::new(input);
    let _iter = searcher.into_matches_iter(|_| Ok(Some(Match::must(0, 0..1))));
}

#[test]
fn test_into_matches_iter_with_anchored_and_earliest() {
    let haystack: &[u8] = b"2021-09-08";
    let input = Input {
        haystack,
        span: Span::new(0, 11),
        anchored: Anchored::True,
        earliest: true,
    };

    let searcher = Searcher::new(input);
    let _iter = searcher.into_matches_iter(|_| Ok(Some(Match::must(0, 0..10))));
}

