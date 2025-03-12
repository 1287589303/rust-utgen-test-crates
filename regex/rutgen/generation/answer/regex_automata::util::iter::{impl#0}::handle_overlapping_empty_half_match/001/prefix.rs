// Answer 0

#[test]
fn test_handle_overlapping_empty_half_match_basic() {
    let haystack: &[u8] = b"example";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: 7 })
        .anchored(Anchored::No)
        .earliest(false);
    let last_match_end = Some(7);
    let half_match = HalfMatch { pattern: PatternID(0), offset: 7 };
    
    let mut searcher = Searcher::new(input);
    searcher.last_match_end = last_match_end;

    let finder = |input: &Input| -> Result<Option<HalfMatch>, MatchError> {
        Ok(None)
    };
    
    let _ = searcher.handle_overlapping_empty_half_match(half_match, finder);
}

#[test]
fn test_handle_overlapping_empty_half_match_edge_case_start() {
    let haystack: &[u8] = b"start";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: 5 })
        .anchored(Anchored::No)
        .earliest(true);
    let last_match_end = Some(0);
    let half_match = HalfMatch { pattern: PatternID(0), offset: 0 };

    let mut searcher = Searcher::new(input);
    searcher.last_match_end = last_match_end;

    let finder = |input: &Input| -> Result<Option<HalfMatch>, MatchError> {
        Ok(Some(HalfMatch { pattern: PatternID(1), offset: 1 }))
    };

    let _ = searcher.handle_overlapping_empty_half_match(half_match, finder);
}

#[test]
fn test_handle_overlapping_empty_half_match_edge_case_empty_haystack() {
    let haystack: &[u8] = b"";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: 0 })
        .anchored(Anchored::No)
        .earliest(false);
    let last_match_end = Some(0);
    let half_match = HalfMatch { pattern: PatternID(0), offset: 0 };

    let mut searcher = Searcher::new(input);
    searcher.last_match_end = last_match_end;

    let finder = |input: &Input| -> Result<Option<HalfMatch>, MatchError> {
        Ok(None)
    };

    let _ = searcher.handle_overlapping_empty_half_match(half_match, finder);
}

#[test]
fn test_handle_overlapping_empty_half_match_edge_case_last_match_end_out_of_bounds() {
    let haystack: &[u8] = b"out_of_bounds";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: 13 })
        .anchored(Anchored::No)
        .earliest(false);
    let last_match_end = Some(14); // Out of bounds
    let half_match = HalfMatch { pattern: PatternID(0), offset: 14 };

    let mut searcher = Searcher::new(input);
    searcher.last_match_end = last_match_end;

    let finder = |input: &Input| -> Result<Option<HalfMatch>, MatchError> {
        Ok(Some(HalfMatch { pattern: PatternID(1), offset: 15 }))
    };

    let _ = searcher.handle_overlapping_empty_half_match(half_match, finder);
}

