// Answer 0

#[test]
fn test_try_search_rev_some_match() {
    let dfa = DFA::builder()
        .thompson(thompson::Config::new().reverse(true))
        .build("foo[0-9]+").unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new("foo12345");
    let expected = HalfMatch::must(0, 0);
    let result = dfa.try_search_rev(&mut cache, &input);
}

#[test]
fn test_try_search_rev_none_match() {
    let dfa = DFA::builder()
        .thompson(thompson::Config::new().reverse(true))
        .build("bar[0-9]+").unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new("foo12345");
    let result = dfa.try_search_rev(&mut cache, &input);
}

#[test]
fn test_try_search_rev_utf8_mode() {
    let dfa = DFA::builder()
        .thompson(thompson::Config::new().reverse(true).utf8(true))
        .build("☃").unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new("☃");
    let matches: Vec<HalfMatch> = Vec::new();
    
    loop {
        match dfa.try_search_rev(&mut cache, &input) {
            Err(_) => break,
            Some(hm) => {
                matches.push(hm);
                if hm.offset() == 0 || input.end() == 0 {
                    break;
                } else {
                    input.set_end(hm.offset());
                }
            }
        }
    }
}

#[test]
fn test_try_search_rev_no_zero_width_matches() {
    let dfa = DFA::builder()
        .thompson(thompson::Config::new().reverse(true).utf8(false))
        .build("☃").unwrap();
    let mut cache = dfa.create_cache();
    let input = Input::new("☃");
    let matches: Vec<HalfMatch> = Vec::new();
    
    loop {
        match dfa.try_search_rev(&mut cache, &input) {
            Err(_) => break,
            Some(hm) => {
                matches.push(hm);
                if hm.offset() == 0 || input.end() == 0 {
                    break;
                } else {
                    input.set_end(hm.offset());
                }
            }
        }
    }
}

