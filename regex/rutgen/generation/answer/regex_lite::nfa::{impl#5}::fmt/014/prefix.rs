// Answer 0

#[test]
fn test_state_ranges_output() {
    let target: StateID = 5;
    let ranges: Vec<(char, char)> = vec![('a', 'z'), ('A', 'Z')];
    let state = State::Ranges { target, ranges };
    let mut output = String::new();
    let mut formatter = core::fmt::Formatter::new(&mut output);
    
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_state_ranges_output_multiple_ranges() {
    let target: StateID = 3;
    let ranges: Vec<(char, char)> = vec![('0', '9'), ('G', 'Z'), ('a', 'f')];
    let state = State::Ranges { target, ranges };
    let mut output = String::new();
    let mut formatter = core::fmt::Formatter::new(&mut output);
    
    let _ = state.fmt(&mut formatter);
}

#[test]
fn test_state_ranges_output_err_case() {
    let target: StateID = 4;
    let ranges: Vec<(char, char)> = vec![('x', 'y'), ('A', 'B')];
    let state = State::Ranges { target, ranges };
    let mut output = String::new();
    let mut formatter = core::fmt::Formatter::new(&mut output);
    
    // Intentionally trigger an err case in the write! to replicate conditions
    let _ = state.fmt(&mut formatter);
}

