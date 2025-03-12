// Answer 0

#[test]
fn test_get_none_engine() {
    let input = Input::new(&b"test"[..])
        .anchored(Anchored::No);
    let one_pass = OnePass(None);
    let result = one_pass.get(&input);
}

#[test]
fn test_get_no_anchored() {
    let nfa = NFA::never_match(); 
    let regex_info = RegexInfo::default(); 
    let one_pass_engine = OnePassEngine::new(&regex_info, &nfa).unwrap();
    let input = Input::new(&b"test"[..])
        .anchored(Anchored::No);
    let one_pass = OnePass(Some(one_pass_engine));
    let result = one_pass.get(&input);
}

