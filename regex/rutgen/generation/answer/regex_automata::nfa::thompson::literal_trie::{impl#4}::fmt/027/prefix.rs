// Answer 0

#[test]
fn test_fmt_empty_state() {
    let state: State = Default::default(); // State with no chunks and no transitions
    let mut output = Vec::new();
    let result = state.fmt(&mut output);
}

#[test]
fn test_fmt_single_chunk() {
    let transition = Transition { start: 0, end: 0, next: StateID::default() };
    let mut state = State {
        transitions: vec![],
        chunks: vec![(0, 1)], // One chunk
    };
    state.transitions.push(transition);
    
    let mut output = Vec::new();
    let result = state.fmt(&mut output);
} 

#[test]
fn test_fmt_single_transition_err() {
    let transition = Transition { start: 0, end: 0, next: StateID::default() };
    let mut state = State {
        transitions: vec![transition],
        chunks: vec![(0, 1)], // One chunk
    };

    let mut output = Vec::new();
    let result = state.fmt(&mut output);
}

