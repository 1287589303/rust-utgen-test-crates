// Answer 0

#[test]
fn test_state_ranges_single_tuple() {
    let target: StateID = 1;
    let ranges: Vec<(char, char)> = vec![('a', 'z')];
    let state = State::Ranges { target, ranges };
    
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state); // Call fmt indirectly for testing
}

#[test]
fn test_state_ranges_multiple_tuples() {
    let target: StateID = 2;
    let ranges: Vec<(char, char)> = vec![('a', 'm'), ('n', 'z')];
    let state = State::Ranges { target, ranges };
    
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state); // Call fmt indirectly for testing
}

#[test]
fn test_state_ranges_empty_ranges() {
    let target: StateID = 3;
    let ranges: Vec<(char, char)> = vec![];
    let state = State::Ranges { target, ranges };
    
    let mut output = String::new();
    let _ = write!(&mut output, "{:?}", state); // Call fmt indirectly for testing
}

