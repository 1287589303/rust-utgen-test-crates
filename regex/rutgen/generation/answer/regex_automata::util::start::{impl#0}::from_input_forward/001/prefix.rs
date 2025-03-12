// Answer 0

#[test]
fn test_from_input_forward_with_no_look_behind() {
    let haystack = b"test string";
    let input = Input::new(&haystack)
        .span(0..4) // start at the beginning
        .anchored(Anchored::No);
    let config = Config::from_input_forward(&input);
}

#[test]
fn test_from_input_forward_with_look_behind() {
    let haystack = b"test string";
    let input = Input::new(&haystack)
        .span(1..5) // start after the first byte
        .anchored(Anchored::No);
    let config = Config::from_input_forward(&input);
}

#[test]
fn test_from_input_forward_with_anchored_yes() {
    let haystack = b"test string";
    let input = Input::new(&haystack)
        .span(0..4)
        .anchored(Anchored::Yes);
    let config = Config::from_input_forward(&input);
}

#[test]
fn test_from_input_forward_with_anchored_pattern() {
    struct PatternID;
    let haystack = b"test string";
    let input = Input::new(&haystack)
        .span(0..4)
        .anchored(Anchored::Pattern(PatternID));
    let config = Config::from_input_forward(&input);
}

#[test]
fn test_from_input_forward_with_start_at_end() {
    let haystack = b"test string";
    let input = Input::new(&haystack)
        .span(10..10) // start at the end
        .anchored(Anchored::No);
    let config = Config::from_input_forward(&input);
}

