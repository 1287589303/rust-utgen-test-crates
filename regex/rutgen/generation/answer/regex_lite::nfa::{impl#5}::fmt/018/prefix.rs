// Answer 0

#[test]
fn test_state_fmt_ranges_empty() {
    let target: StateID = 1;
    let ranges: Vec<(char, char)> = vec![];
    let state = State::Ranges { target, ranges };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

#[test]
fn test_state_fmt_ranges_non_empty() {
    let target: StateID = 2;
    let ranges: Vec<(char, char)> = vec![('a', 'z')];
    let state = State::Ranges { target, ranges };
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state);
}

