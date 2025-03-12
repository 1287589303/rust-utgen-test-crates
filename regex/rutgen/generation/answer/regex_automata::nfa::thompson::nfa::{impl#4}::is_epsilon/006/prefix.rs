// Answer 0

#[test]
fn test_is_epsilon_look_start() {
    let state = State::Look {
        look: Look::Start,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_end() {
    let state = State::Look {
        look: Look::End,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_startlf() {
    let state = State::Look {
        look: Look::StartLF,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_endlf() {
    let state = State::Look {
        look: Look::EndLF,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_wordascii() {
    let state = State::Look {
        look: Look::WordAscii,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_wordunicode() {
    let state = State::Look {
        look: Look::WordUnicode,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_wordstartascii() {
    let state = State::Look {
        look: Look::WordStartAscii,
        next: StateID::default(),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_look_wordendascii() {
    let state = State::Look {
        look: Look::WordEndAscii,
        next: StateID::default(),
    };
    state.is_epsilon();
}

