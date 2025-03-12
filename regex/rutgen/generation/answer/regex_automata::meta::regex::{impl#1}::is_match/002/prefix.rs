// Answer 0

#[test]
fn test_is_match_with_valid_input() {
    let re = Regex::new("a+").unwrap();
    let input = Input::new("aaaaa").span(0..5).anchored(Anchored::No);
    assert!(re.is_match(input));
}

#[test]
fn test_is_match_with_valid_input_1() {
    let re = Regex::new("foo[0-9]+bar").unwrap();
    let input = Input::new("foo12345bar").span(0..15).anchored(Anchored::No);
    assert!(re.is_match(input));
}

#[test]
fn test_is_match_with_valid_input_2() {
    let re = Regex::new(r"(?m:$)").unwrap();
    let input = Input::new("\n").span(0..1).anchored(Anchored::No);
    assert!(re.is_match(input));
}

#[test]
fn test_is_match_with_valid_span() {
    let re = Regex::new("b*").unwrap();
    let input = Input::new("aaaaabbbb").span(5..9).anchored(Anchored::No);
    assert!(re.is_match(input));
}

#[test]
fn test_is_match_with_earliest_true() {
    let re = Regex::new("a+").unwrap();
    let input = Input::new("aaa").span(0..3).anchored(Anchored::No).earliest(true);
    assert!(re.is_match(input));
}

#[test]
fn test_is_match_empty_haystack() {
    let re = Regex::new(".*").unwrap();
    let input = Input::new("").span(0..0).anchored(Anchored::No);
    assert!(re.is_match(input));
}

