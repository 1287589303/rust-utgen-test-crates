// Answer 0

#[test]
fn test_try_search_valid_inputs_anchored_yes() {
    let re = DFA::builder()
        .configure(DFA::config().starts_for_each_pattern(true))
        .build_many(&["[a-z]+", "[0-9]+"]).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    let haystack = "123abc";
    let input = Input::new(haystack).anchored(Anchored::Yes);
    re.try_search(&mut cache, &input, &mut caps).unwrap();
}

#[test]
fn test_try_search_valid_inputs_pattern() {
    let re = DFA::builder()
        .configure(DFA::config().starts_for_each_pattern(true))
        .build_many(&["[a-z]+", "[0-9]+"]).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    let haystack = "123abc";
    let input = Input::new(haystack).anchored(Anchored::Pattern(PatternID::must(1))); // pattern for digits
    re.try_search(&mut cache, &input, &mut caps).unwrap();
}

#[test]
fn test_try_search_with_empty_captures() {
    let re = DFA::builder()
        .configure(DFA::config().starts_for_each_pattern(true))
        .build_many(&["a+"]).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    caps.clear(); // start with cleared captures
    let haystack = "aaa";
    let input = Input::new(haystack).anchored(Anchored::Yes);
    re.try_search(&mut cache, &input, &mut caps).unwrap();
}

#[test]
fn test_try_search_with_slots_minimum() {
    let re = DFA::builder()
        .configure(DFA::config().starts_for_each_pattern(true))
        .build_many(&["ab"]).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    let haystack = "ab";
    let input = Input::new(haystack).anchored(Anchored::Yes);
    re.try_search(&mut cache, &input, &mut caps).unwrap();
}

#[test]
fn test_try_search_with_long_haystack() {
    let re = DFA::builder()
        .configure(DFA::config().starts_for_each_pattern(true))
        .build_many(&["a+", "b+"]).unwrap();
    let (mut cache, mut caps) = (re.create_cache(), re.create_captures());
    let haystack = "aaabbb";
    let input = Input::new(haystack).anchored(Anchored::Yes);
    re.try_search(&mut cache, &input, &mut caps).unwrap();
}

