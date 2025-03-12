// Answer 0

#[test]
fn test_is_epsilon_binary_union() {
    use regex_automata::util::primitives::{StateID};

    let state = State::BinaryUnion {
        alt1: StateID(SmallIndex(1)),
        alt2: StateID(SmallIndex(2)),
    };
    state.is_epsilon();
}

#[test]
fn test_is_epsilon_binary_union_with_zero_ids() {
    use regex_automata::util::primitives::{StateID};

    let state = State::BinaryUnion {
        alt1: StateID(SmallIndex(0)),
        alt2: StateID(SmallIndex(0)),
    };
    state.is_epsilon();
}

