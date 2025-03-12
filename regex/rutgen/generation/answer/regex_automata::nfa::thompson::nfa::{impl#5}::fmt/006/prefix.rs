// Answer 0

#[test]
fn test_state_look_debug() {
    let look = Look::Start;
    let next = StateID(SmallIndex::new_unchecked(5));
    let state = State::Look { look, next };
    let mut output = String::new();
    let res = state.fmt(&mut output);
}

#[test]
fn test_state_look_debug_end() {
    let look = Look::End;
    let next = StateID(SmallIndex::new_unchecked(10));
    let state = State::Look { look, next };
    let mut output = String::new();
    let res = state.fmt(&mut output);
}

#[test]
fn test_state_look_debug_word_ascii() {
    let look = Look::WordAscii;
    let next = StateID(SmallIndex::new_unchecked(15));
    let state = State::Look { look, next };
    let mut output = String::new();
    let res = state.fmt(&mut output);
}

#[test]
fn test_state_look_debug_word_unicode() {
    let look = Look::WordUnicode;
    let next = StateID(SmallIndex::new_unchecked(20));
    let state = State::Look { look, next };
    let mut output = String::new();
    let res = state.fmt(&mut output);
}

#[test]
fn test_state_look_debug_large_next() {
    let look = Look::WordEndAscii;
    let next = StateID(SmallIndex::new_unchecked(core::i32::MAX as usize - 1));
    let state = State::Look { look, next };
    let mut output = String::new();
    let res = state.fmt(&mut output);
}

