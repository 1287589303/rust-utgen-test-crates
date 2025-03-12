pub(crate) fn next(
    nfa: &thompson::NFA,
    match_kind: MatchKind,
    sparses: &mut SparseSets,
    stack: &mut Vec<StateID>,
    state: &State,
    unit: alphabet::Unit,
    empty_builder: StateBuilderEmpty,
) -> StateBuilderNFA {
    sparses.clear();

    // Whether the NFA is matched in reverse or not. We use this in some
    // conditional logic for dealing with the exceptionally annoying CRLF-aware
    // line anchors.
    let rev = nfa.is_reverse();
    // The look-around matcher that our NFA is configured with. We don't
    // actually use it to match look-around assertions, but we do need its
    // configuration for constructing states consistent with how it matches.
    let lookm = nfa.look_matcher();

    // Put the NFA state IDs into a sparse set in case we need to
    // re-compute their epsilon closure.
    //
    // Doing this state shuffling is technically not necessary unless some
    // kind of look-around is used in the DFA. Some ad hoc experiments
    // suggested that avoiding this didn't lead to much of an improvement,
    // but perhaps more rigorous experimentation should be done. And in
    // particular, avoiding this check requires some light refactoring of
    // the code below.
    state.iter_nfa_state_ids(|nfa_id| {
        sparses.set1.insert(nfa_id);
    });

    // Compute look-ahead assertions originating from the current state. Based
    // on the input unit we're transitioning over, some additional set of
    // assertions may be true. Thus, we re-compute this state's epsilon closure
    // (but only if necessary). Notably, when we build a DFA state initially,
    // we don't enable any look-ahead assertions because we don't know whether
    // they're true or not at that point.
    if !state.look_need().is_empty() {
        // Add look-ahead assertions that are now true based on the current
        // input unit.
        let mut look_have = state.look_have().clone();
        match unit.as_u8() {
            Some(b'\r') => {
                if !rev || !state.is_half_crlf() {
                    look_have = look_have.insert(Look::EndCRLF);
                }
            }
            Some(b'\n') => {
                if rev || !state.is_half_crlf() {
                    look_have = look_have.insert(Look::EndCRLF);
                }
            }
            Some(_) => {}
            None => {
                look_have = look_have
                    .insert(Look::End)
                    .insert(Look::EndLF)
                    .insert(Look::EndCRLF);
            }
        }
        if unit.is_byte(lookm.get_line_terminator()) {
            look_have = look_have.insert(Look::EndLF);
        }
        if state.is_half_crlf()
            && ((rev && !unit.is_byte(b'\r'))
                || (!rev && !unit.is_byte(b'\n')))
        {
            look_have = look_have.insert(Look::StartCRLF);
        }
        if state.is_from_word() == unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordAsciiNegate)
                .insert(Look::WordUnicodeNegate);
        } else {
            look_have =
                look_have.insert(Look::WordAscii).insert(Look::WordUnicode);
        }
        if !unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordEndHalfAscii)
                .insert(Look::WordEndHalfUnicode);
        }
        if state.is_from_word() && !unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordEndAscii)
                .insert(Look::WordEndUnicode);
        } else if !state.is_from_word() && unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordStartAscii)
                .insert(Look::WordStartUnicode);
        }
        // If we have new assertions satisfied that are among the set of
        // assertions that exist in this state (that is, just because we added
        // an EndLF assertion above doesn't mean there is an EndLF conditional
        // epsilon transition in this state), then we re-compute this state's
        // epsilon closure using the updated set of assertions.
        //
        // Note that since our DFA states omit unconditional epsilon
        // transitions, this check is necessary for correctness. If we re-did
        // the epsilon closure below needlessly, it could change based on the
        // fact that we omitted epsilon states originally.
        if !look_have
            .subtract(state.look_have())
            .intersect(state.look_need())
            .is_empty()
        {
            for nfa_id in sparses.set1.iter() {
                epsilon_closure(
                    nfa,
                    nfa_id,
                    look_have,
                    stack,
                    &mut sparses.set2,
                );
            }
            sparses.swap();
            sparses.set2.clear();
        }
    }

    // Convert our empty builder into one that can record assertions and match
    // pattern IDs.
    let mut builder = empty_builder.into_matches();
    // Set whether the StartLF look-behind assertion is true for this
    // transition or not. The look-behind assertion for ASCII word boundaries
    // is handled below.
    if nfa.look_set_any().contains_anchor_line()
        && unit.is_byte(lookm.get_line_terminator())
    {
        // Why only handle StartLF here and not Start? That's because Start
        // can only impact the starting state, which is special cased in
        // start state handling.
        builder.set_look_have(|have| have.insert(Look::StartLF));
    }
    // We also need to add StartCRLF to our assertions too, if we can. This
    // is unfortunately a bit more complicated, because it depends on the
    // direction of the search. In the forward direction, ^ matches after a
    // \n, but in the reverse direction, ^ only matches after a \r. (This is
    // further complicated by the fact that reverse a regex means changing a ^
    // to a $ and vice versa.)
    if nfa.look_set_any().contains_anchor_crlf()
        && ((rev && unit.is_byte(b'\r')) || (!rev && unit.is_byte(b'\n')))
    {
        builder.set_look_have(|have| have.insert(Look::StartCRLF));
    }
    // And also for the start-half word boundary assertions. As long as the
    // look-behind byte is not a word char, then the assertions are satisfied.
    if nfa.look_set_any().contains_word() && !unit.is_word_byte() {
        builder.set_look_have(|have| {
            have.insert(Look::WordStartHalfAscii)
                .insert(Look::WordStartHalfUnicode)
        });
    }
    for nfa_id in sparses.set1.iter() {
        match *nfa.state(nfa_id) {
            thompson::State::Union { .. }
            | thompson::State::BinaryUnion { .. }
            | thompson::State::Fail
            | thompson::State::Look { .. }
            | thompson::State::Capture { .. } => {}
            thompson::State::Match { pattern_id } => {
                // Notice here that we are calling the NEW state a match
                // state if the OLD state we are transitioning from
                // contains an NFA match state. This is precisely how we
                // delay all matches by one byte and also what therefore
                // guarantees that starting states cannot be match states.
                //
                // If we didn't delay matches by one byte, then whether
                // a DFA is a matching state or not would be determined
                // by whether one of its own constituent NFA states
                // was a match state. (And that would be done in
                // 'add_nfa_states'.)
                //
                // Also, 'add_match_pattern_id' requires that callers never
                // pass duplicative pattern IDs. We do in fact uphold that
                // guarantee here, but it's subtle. In particular, a Thompson
                // NFA guarantees that each pattern has exactly one match
                // state. Moreover, since we're iterating over the NFA state
                // IDs in a set, we are guarateed not to have any duplicative
                // match states. Thus, it is impossible to add the same pattern
                // ID more than once.
                //
                // N.B. We delay matches by 1 byte as a way to hack 1-byte
                // look-around into DFA searches. This lets us support ^, $
                // and ASCII-only \b. The delay is also why we need a special
                // "end-of-input" (EOI) sentinel and why we need to follow the
                // EOI sentinel at the end of every search. This final EOI
                // transition is necessary to report matches found at the end
                // of a haystack.
                builder.add_match_pattern_id(pattern_id);
                if !match_kind.continue_past_first_match() {
                    break;
                }
            }
            thompson::State::ByteRange { ref trans } => {
                if trans.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        trans.next,
                        builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
            thompson::State::Sparse(ref sparse) => {
                if let Some(next) = sparse.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
            thompson::State::Dense(ref dense) => {
                if let Some(next) = dense.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
        }
    }
    // We only set the word byte if there's a word boundary look-around
    // anywhere in this regex. Otherwise, there's no point in bloating the
    // number of states if we don't have one.
    //
    // We also only set it when the state has a non-zero number of NFA states.
    // Otherwise, we could wind up with states that *should* be DEAD states
    // but are otherwise distinct from DEAD states because of this look-behind
    // assertion being set. While this can't technically impact correctness *in
    // theory*, it can create pathological DFAs that consume input until EOI or
    // a quit byte is seen. Consuming until EOI isn't a correctness problem,
    // but a (serious) perf problem. Hitting a quit byte, however, could be a
    // correctness problem since it could cause search routines to report an
    // error instead of a detected match once the quit state is entered. (The
    // search routine could be made to be a bit smarter by reporting a match
    // if one was detected once it enters a quit state (and indeed, the search
    // routines in this crate do just that), but it seems better to prevent
    // these things by construction if possible.)
    if !sparses.set2.is_empty() {
        if nfa.look_set_any().contains_word() && unit.is_word_byte() {
            builder.set_is_from_word();
        }
        if nfa.look_set_any().contains_anchor_crlf()
            && ((rev && unit.is_byte(b'\n')) || (!rev && unit.is_byte(b'\r')))
        {
            builder.set_is_half_crlf();
        }
    }
    let mut builder_nfa = builder.into_nfa();
    add_nfa_states(nfa, &sparses.set2, &mut builder_nfa);
    builder_nfa
}