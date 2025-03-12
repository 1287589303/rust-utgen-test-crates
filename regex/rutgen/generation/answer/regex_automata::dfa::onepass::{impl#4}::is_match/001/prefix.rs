// Answer 0

#[test]
fn test_is_match_true_anchored_yes() {
    let re = DFA::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foo12345bar"[..]).span(0..15).anchored(Anchored::Yes);
    let result = re.is_match(&mut cache, input);
}

#[test]
fn test_is_match_false_anchored_yes() {
    let re = DFA::new("foo[0-9]+bar").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"foobar"[..]).span(0..6).anchored(Anchored::Yes);
    let result = re.is_match(&mut cache, input);
}

#[test]
fn test_is_match_true_anchored_pattern() {
    let re = DFA::new("a+").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"aaa"[..]).span(0..3).anchored(Anchored::Pattern(PatternID(0)));
    let result = re.is_match(&mut cache, input);
}

#[test]
fn test_is_match_false_anchored_pattern() {
    let re = DFA::new("a+").unwrap();
    let mut cache = re.create_cache();
    let input = Input::new(&b"bbb"[..]).span(0..3).anchored(Anchored::Pattern(PatternID(0)));
    let result = re.is_match(&mut cache, input);
}

