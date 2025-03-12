pub(crate) fn epsilon_closure(
    nfa: &thompson::NFA,
    start_nfa_id: StateID,
    look_have: LookSet,
    stack: &mut Vec<StateID>,
    set: &mut SparseSet,
) {
    assert!(stack.is_empty());
    // If this isn't an epsilon state, then the epsilon closure is always just
    // itself, so there's no need to spin up the machinery below to handle it.
    if !nfa.state(start_nfa_id).is_epsilon() {
        set.insert(start_nfa_id);
        return;
    }

    stack.push(start_nfa_id);
    while let Some(mut id) = stack.pop() {
        // In many cases, we can avoid stack operations when an NFA state only
        // adds one new state to visit. In that case, we just set our ID to
        // that state and mush on. We only use the stack when an NFA state
        // introduces multiple new states to visit.
        loop {
            // Insert this NFA state, and if it's already in the set and thus
            // already visited, then we can move on to the next one.
            if !set.insert(id) {
                break;
            }
            match *nfa.state(id) {
                thompson::State::ByteRange { .. }
                | thompson::State::Sparse { .. }
                | thompson::State::Dense { .. }
                | thompson::State::Fail
                | thompson::State::Match { .. } => break,
                thompson::State::Look { look, next } => {
                    if !look_have.contains(look) {
                        break;
                    }
                    id = next;
                }
                thompson::State::Union { ref alternates } => {
                    id = match alternates.get(0) {
                        None => break,
                        Some(&id) => id,
                    };
                    // We need to process our alternates in order to preserve
                    // match preferences, so put the earliest alternates closer
                    // to the top of the stack.
                    stack.extend(alternates[1..].iter().rev());
                }
                thompson::State::BinaryUnion { alt1, alt2 } => {
                    id = alt1;
                    stack.push(alt2);
                }
                thompson::State::Capture { next, .. } => {
                    id = next;
                }
            }
        }
    }
}