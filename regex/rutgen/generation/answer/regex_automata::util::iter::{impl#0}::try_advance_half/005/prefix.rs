// Answer 0

#[test]
fn test_try_advance_half_ok_some() {
    let haystack: &[u8] = b"example string for matching";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let mut searcher = Searcher::new(input);
    
    let result = searcher.try_advance_half(|input| {
        let half_match = HalfMatch::new(PatternID::new(1), 7);
        Ok(Some(half_match))
    });
}

#[test]
fn test_try_advance_half_err_none() {
    let haystack: &[u8] = b"example string for matching";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let mut searcher = Searcher::new(input);
    
    let result = searcher.try_advance_half(|input| {
        Err(MatchError::new(/* appropriate MatchErrorKind */))
    });
}

#[test]
fn test_try_advance_half_ok_some_last_match_end_none() {
    let haystack: &[u8] = b"example string for matching";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let mut searcher = Searcher::new(input);
    searcher.last_match_end = None;

    let result = searcher.try_advance_half(|input| {
        let half_match = HalfMatch::new(PatternID::new(1), 5);
        Ok(Some(half_match))
    });
}

#[test]
fn test_try_advance_half_ok_some_last_match_end_some_false() {
    let haystack: &[u8] = b"example string for matching";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let mut searcher = Searcher::new(input);
    searcher.last_match_end = Some(6); // different from matched offset

    let result = searcher.try_advance_half(|input| {
        let half_match = HalfMatch::new(PatternID::new(1), 7); // offset does not match last_match_end
        Ok(Some(half_match))
    });
}

