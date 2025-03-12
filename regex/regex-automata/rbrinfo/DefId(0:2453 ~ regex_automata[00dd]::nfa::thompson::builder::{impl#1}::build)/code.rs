pub fn build(
        &self,
        start_anchored: StateID,
        start_unanchored: StateID,
    ) -> Result<NFA, BuildError> {
        assert!(self.pattern_id.is_none(), "must call 'finish_pattern' first");
        debug!(
            "intermediate NFA compilation via builder is complete, \
             intermediate NFA size: {} states, {} bytes on heap",
            self.states.len(),
            self.memory_usage(),
        );

        let mut nfa = nfa::Inner::default();
        nfa.set_utf8(self.utf8);
        nfa.set_reverse(self.reverse);
        nfa.set_look_matcher(self.look_matcher.clone());
        // A set of compiler internal state IDs that correspond to states
        // that are exclusively epsilon transitions, i.e., goto instructions,
        // combined with the state that they point to. This is used to
        // record said states while transforming the compiler's internal NFA
        // representation to the external form.
        let mut empties = vec![];
        // A map used to re-map state IDs when translating this builder's
        // internal NFA state representation to the final NFA representation.
        let mut remap = vec![];
        remap.resize(self.states.len(), StateID::ZERO);

        nfa.set_starts(start_anchored, start_unanchored, &self.start_pattern);
        nfa.set_captures(&self.captures).map_err(BuildError::captures)?;
        // The idea here is to convert our intermediate states to their final
        // form. The only real complexity here is the process of converting
        // transitions, which are expressed in terms of state IDs. The new
        // set of states will be smaller because of partial epsilon removal,
        // so the state IDs will not be the same.
        for (sid, state) in self.states.iter().with_state_ids() {
            match *state {
                State::Empty { next } => {
                    // Since we're removing empty states, we need to handle
                    // them later since we don't yet know which new state this
                    // empty state will be mapped to.
                    empties.push((sid, next));
                }
                State::ByteRange { trans } => {
                    remap[sid] = nfa.add(nfa::State::ByteRange { trans });
                }
                State::Sparse { ref transitions } => {
                    remap[sid] = match transitions.len() {
                        0 => nfa.add(nfa::State::Fail),
                        1 => nfa.add(nfa::State::ByteRange {
                            trans: transitions[0],
                        }),
                        _ => {
                            let transitions =
                                transitions.to_vec().into_boxed_slice();
                            let sparse = SparseTransitions { transitions };
                            nfa.add(nfa::State::Sparse(sparse))
                        }
                    }
                }
                State::Look { look, next } => {
                    remap[sid] = nfa.add(nfa::State::Look { look, next });
                }
                State::CaptureStart { pattern_id, group_index, next } => {
                    // We can't remove this empty state because of the side
                    // effect of capturing an offset for this capture slot.
                    let slot = nfa
                        .group_info()
                        .slot(pattern_id, group_index.as_usize())
                        .expect("invalid capture index");
                    let slot =
                        SmallIndex::new(slot).expect("a small enough slot");
                    remap[sid] = nfa.add(nfa::State::Capture {
                        next,
                        pattern_id,
                        group_index,
                        slot,
                    });
                }
                State::CaptureEnd { pattern_id, group_index, next } => {
                    // We can't remove this empty state because of the side
                    // effect of capturing an offset for this capture slot.
                    // Also, this always succeeds because we check that all
                    // slot indices are valid for all capture indices when they
                    // are initially added.
                    let slot = nfa
                        .group_info()
                        .slot(pattern_id, group_index.as_usize())
                        .expect("invalid capture index")
                        .checked_add(1)
                        .unwrap();
                    let slot =
                        SmallIndex::new(slot).expect("a small enough slot");
                    remap[sid] = nfa.add(nfa::State::Capture {
                        next,
                        pattern_id,
                        group_index,
                        slot,
                    });
                }
                State::Union { ref alternates } => {
                    if alternates.is_empty() {
                        remap[sid] = nfa.add(nfa::State::Fail);
                    } else if alternates.len() == 1 {
                        empties.push((sid, alternates[0]));
                        remap[sid] = alternates[0];
                    } else if alternates.len() == 2 {
                        remap[sid] = nfa.add(nfa::State::BinaryUnion {
                            alt1: alternates[0],
                            alt2: alternates[1],
                        });
                    } else {
                        let alternates =
                            alternates.to_vec().into_boxed_slice();
                        remap[sid] = nfa.add(nfa::State::Union { alternates });
                    }
                }
                State::UnionReverse { ref alternates } => {
                    if alternates.is_empty() {
                        remap[sid] = nfa.add(nfa::State::Fail);
                    } else if alternates.len() == 1 {
                        empties.push((sid, alternates[0]));
                        remap[sid] = alternates[0];
                    } else if alternates.len() == 2 {
                        remap[sid] = nfa.add(nfa::State::BinaryUnion {
                            alt1: alternates[1],
                            alt2: alternates[0],
                        });
                    } else {
                        let mut alternates =
                            alternates.to_vec().into_boxed_slice();
                        alternates.reverse();
                        remap[sid] = nfa.add(nfa::State::Union { alternates });
                    }
                }
                State::Fail => {
                    remap[sid] = nfa.add(nfa::State::Fail);
                }
                State::Match { pattern_id } => {
                    remap[sid] = nfa.add(nfa::State::Match { pattern_id });
                }
            }
        }
        // Some of the new states still point to empty state IDs, so we need to
        // follow each of them and remap the empty state IDs to their non-empty
        // state IDs.
        //
        // We also keep track of which states we've already mapped. This helps
        // avoid quadratic behavior in a long chain of empty states. For
        // example, in 'a{0}{50000}'.
        let mut remapped = vec![false; self.states.len()];
        for &(empty_id, empty_next) in empties.iter() {
            if remapped[empty_id] {
                continue;
            }
            // empty states can point to other empty states, forming a chain.
            // So we must follow the chain until the end, which must end at
            // a non-empty state, and therefore, a state that is correctly
            // remapped. We are guaranteed to terminate because our compiler
            // never builds a loop among only empty states.
            let mut new_next = empty_next;
            while let Some(next) = self.states[new_next].goto() {
                new_next = next;
            }
            remap[empty_id] = remap[new_next];
            remapped[empty_id] = true;

            // Now that we've remapped the main 'empty_id' above, we re-follow
            // the chain from above and remap every empty state we found along
            // the way to our ultimate non-empty target. We are careful to set
            // 'remapped' to true for each such state. We thus will not need
            // to re-compute this chain for any subsequent empty states in
            // 'empties' that are part of this chain.
            let mut next2 = empty_next;
            while let Some(next) = self.states[next2].goto() {
                remap[next2] = remap[new_next];
                remapped[next2] = true;
                next2 = next;
            }
        }
        // Finally remap all of the state IDs.
        nfa.remap(&remap);
        let final_nfa = nfa.into_nfa();
        debug!(
            "NFA compilation via builder complete, \
             final NFA size: {} states, {} bytes on heap, \
             has empty? {:?}, utf8? {:?}",
            final_nfa.states().len(),
            final_nfa.memory_usage(),
            final_nfa.has_empty(),
            final_nfa.is_utf8(),
        );
        Ok(final_nfa)
    }