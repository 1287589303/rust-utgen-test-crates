// Answer 0

#[test]
fn test_fmt_state_ranges_with_single_range() {
    let target: StateID = 1; 
    let ranges = vec![('a', 'b')]; 
    let state = State::Ranges { target, ranges }; 
    let mut buffer = String::new(); 
    let result = write!(&mut buffer, "{:?}", state);
}

#[test]
fn test_fmt_state_ranges_with_err() {
    let target: StateID = 2; 
    let ranges = vec![('c', 'd')]; 
    let state = State::Ranges { target, ranges }; 
    let mut buffer = String::new(); 
    let result = write!(&mut buffer, "{:?}", state);
}

