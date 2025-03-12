// Answer 0

#[test]
fn test_is_match_with_no_anchored_and_quadratic_error() {
    let core_info = RegexInfo::default();
    let prefilter = Prefilter::default();
    let nfa = NFA::default();
    let pikevm = wrappers::PikeVM::default();
    let backtrack = wrappers::BoundedBacktracker::default();
    let onepass = wrappers::OnePass::default();
    let hybrid = wrappers::Hybrid::default();
    let dfa = wrappers::DFA::default();
    
    let core = Core {
        info: core_info,
        pre: Some(prefilter.clone()),
        nfa,
        nfarev: None,
        pikevm,
        backtrack,
        onepass,
        hybrid,
        dfa,
    };
    
    let strategy = ReverseSuffix { core, pre: prefilter };
    
    let haystack: &[u8] = b"";
    let input = Input::new(haystack).anchored(Anchored::No).span(0..0); // no match case
    let mut cache = strategy.create_cache();

    strategy
        .try_search_half_start(&mut cache, &input)
        .expect_err("expected quadratic error");
}

#[test]
fn test_is_match_with_no_anchored_and_failed_error() {
    let core_info = RegexInfo::default();
    let prefilter = Prefilter::default();
    let nfa = NFA::default();
    let pikevm = wrappers::PikeVM::default();
    let backtrack = wrappers::BoundedBacktracker::default();
    let onepass = wrappers::OnePass::default();
    let hybrid = wrappers::Hybrid::default();
    let dfa = wrappers::DFA::default();

    let core = Core {
        info: core_info,
        pre: Some(prefilter.clone()),
        nfa,
        nfarev: None,
        pikevm,
        backtrack,
        onepass,
        hybrid,
        dfa,
    };

    let strategy = ReverseSuffix { core, pre: prefilter };

    let haystack: &[u8] = b"";
    let input = Input::new(haystack).anchored(Anchored::No).span(0..0); // no match case
    let mut cache = strategy.create_cache();

    strategy
        .try_search_half_start(&mut cache, &input)
        .expect_err("expected failed error");
}

#[test]
fn test_is_match_with_no_anchored_and_half_match_found() {
    let core_info = RegexInfo::default();
    let prefilter = Prefilter::default();
    let nfa = NFA::default();
    let pikevm = wrappers::PikeVM::default();
    let backtrack = wrappers::BoundedBacktracker::default();
    let onepass = wrappers::OnePass::default();
    let hybrid = wrappers::Hybrid::default();
    let dfa = wrappers::DFA::default();

    let core = Core {
        info: core_info,
        pre: Some(prefilter.clone()),
        nfa,
        nfarev: None,
        pikevm,
        backtrack,
        onepass,
        hybrid,
        dfa,
    };

    let strategy = ReverseSuffix { core, pre: prefilter };

    let haystack: &[u8] = b"test haystack";
    let input = Input::new(haystack).anchored(Anchored::No).span(0..haystack.len());
    let mut cache = strategy.create_cache();

    let result = strategy.try_search_half_start(&mut cache, &input);
    match result {
        Ok(Some(_)) => {},
        _ => panic!("Expected half match found")
    }
}

