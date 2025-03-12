// Answer 0

#[test]
fn test_advance_half_valid_match() {
    let haystack: &[u8] = b"2010-03-14 2016-10-08 2020-10-22";
    let span = Span::new(0, haystack.len());
    let anchored = Anchored::Unanchored;
    let earliest = false;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest,
    };
    
    let mut searcher = Searcher::new(input);
    
    let re = DFA::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let mut cache = re.create_cache();

    let _match = searcher.advance_half(|input| re.try_search_fwd(&mut cache, input));
}

#[test]
fn test_advance_half_empty_match() {
    let haystack: &[u8] = b"abba";
    let span = Span::new(0, haystack.len());
    let anchored = Anchored::Unanchored;
    let earliest = false;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest,
    };
    
    let mut searcher = Searcher::new(input);
    
    let re = DFA::new(r"a|").unwrap();
    let mut cache = re.create_cache();
    
    let _match = searcher.advance_half(|input| re.try_search_fwd(&mut cache, input));
}

#[test]
fn test_advance_half_boundary_case() {
    let haystack: &[u8] = b"";
    let span = Span::new(0, 0);
    let anchored = Anchored::Unanchored;
    let earliest = false;
    let input = Input {
        haystack,
        span,
        anchored,
        earliest,
    };
    
    let mut searcher = Searcher::new(input);
    
    let re = DFA::new(r".").unwrap();
    let mut cache = re.create_cache();

    let _match = searcher.advance_half(|input| re.try_search_fwd(&mut cache, input));
}

