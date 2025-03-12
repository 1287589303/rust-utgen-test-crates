fn step(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        mut sid: StateID,
        mut at: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {
        loop {
            if !cache.visited.insert(sid, at - input.start()) {
                return None;
            }
            match *self.nfa.state(sid) {
                State::ByteRange { ref trans } => {
                    // Why do we need this? Unlike other regex engines in this
                    // crate, the backtracker can steam roll ahead in the
                    // haystack outside of the main loop over the bytes in the
                    // haystack. While 'trans.matches()' below handles the case
                    // of 'at' being out of bounds of 'input.haystack()', we
                    // also need to handle the case of 'at' going out of bounds
                    // of the span the caller asked to search.
                    //
                    // We should perhaps make the 'trans.matches()' API accept
                    // an '&Input' instead of a '&[u8]'. Or at least, add a new
                    // API that does it.
                    if at >= input.end() {
                        return None;
                    }
                    if !trans.matches(input.haystack(), at) {
                        return None;
                    }
                    sid = trans.next;
                    at += 1;
                }
                State::Sparse(ref sparse) => {
                    if at >= input.end() {
                        return None;
                    }
                    sid = sparse.matches(input.haystack(), at)?;
                    at += 1;
                }
                State::Dense(ref dense) => {
                    if at >= input.end() {
                        return None;
                    }
                    sid = dense.matches(input.haystack(), at)?;
                    at += 1;
                }
                State::Look { look, next } => {
                    // OK because we don't permit building a searcher with a
                    // Unicode word boundary if the requisite Unicode data is
                    // unavailable.
                    if !self.nfa.look_matcher().matches_inline(
                        look,
                        input.haystack(),
                        at,
                    ) {
                        return None;
                    }
                    sid = next;
                }
                State::Union { ref alternates } => {
                    sid = match alternates.get(0) {
                        None => return None,
                        Some(&sid) => sid,
                    };
                    cache.stack.extend(
                        alternates[1..]
                            .iter()
                            .copied()
                            .rev()
                            .map(|sid| Frame::Step { sid, at }),
                    );
                }
                State::BinaryUnion { alt1, alt2 } => {
                    sid = alt1;
                    cache.stack.push(Frame::Step { sid: alt2, at });
                }
                State::Capture { next, slot, .. } => {
                    if slot.as_usize() < slots.len() {
                        cache.stack.push(Frame::RestoreCapture {
                            slot,
                            offset: slots[slot],
                        });
                        slots[slot] = NonMaxUsize::new(at);
                    }
                    sid = next;
                }
                State::Fail => return None,
                State::Match { pattern_id } => {
                    return Some(HalfMatch::new(pattern_id, at));
                }
            }
        }
    }