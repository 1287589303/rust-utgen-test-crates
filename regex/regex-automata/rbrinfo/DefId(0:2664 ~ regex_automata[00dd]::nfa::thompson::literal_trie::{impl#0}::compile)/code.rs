pub(crate) fn compile(
        &self,
        builder: &mut Builder,
    ) -> Result<ThompsonRef, BuildError> {
        // Compilation proceeds via depth-first traversal of the trie.
        //
        // This is overall pretty brutal. The recursive version of this is
        // deliciously simple. (See 'compile_to_hir' below for what it might
        // look like.) But recursion on a trie means your call stack grows
        // in accordance with the longest literal, which just does not seem
        // appropriate. So we push the call stack to the heap. But as a result,
        // the trie traversal becomes pretty brutal because we essentially
        // have to encode the state of a double for-loop into an explicit call
        // frame. If someone can simplify this without using recursion, that'd
        // be great.

        // 'end' is our match state for this trie, but represented in the the
        // NFA. Any time we see a match in the trie, we insert a transition
        // from the current state we're in to 'end'.
        let end = builder.add_empty()?;
        let mut stack = vec![];
        let mut f = Frame::new(&self.states[StateID::ZERO]);
        loop {
            if let Some(t) = f.transitions.next() {
                if self.states[t.next].is_leaf() {
                    f.sparse.push(thompson::Transition {
                        start: t.byte,
                        end: t.byte,
                        next: end,
                    });
                } else {
                    f.sparse.push(thompson::Transition {
                        start: t.byte,
                        end: t.byte,
                        // This is a little funny, but when the frame we create
                        // below completes, it will pop this parent frame off
                        // and modify this transition to point to the correct
                        // state.
                        next: StateID::ZERO,
                    });
                    stack.push(f);
                    f = Frame::new(&self.states[t.next]);
                }
                continue;
            }
            // At this point, we have visited all transitions in f.chunk, so
            // add it as a sparse NFA state. Unless the chunk was empty, in
            // which case, we don't do anything.
            if !f.sparse.is_empty() {
                let chunk_id = if f.sparse.len() == 1 {
                    builder.add_range(f.sparse.pop().unwrap())?
                } else {
                    let sparse = mem::replace(&mut f.sparse, vec![]);
                    builder.add_sparse(sparse)?
                };
                f.union.push(chunk_id);
            }
            // Now we need to look to see if there are other chunks to visit.
            if let Some(chunk) = f.chunks.next() {
                // If we're here, it means we're on the second (or greater)
                // chunk, which implies there is a match at this point. So
                // connect this state to the final end state.
                f.union.push(end);
                // Advance to the next chunk.
                f.transitions = chunk.iter();
                continue;
            }
            // Now that we are out of chunks, we have completely visited
            // this state. So turn our union of chunks into an NFA union
            // state, and add that union state to the parent state's current
            // sparse state. (If there is no parent, we're done.)
            let start = builder.add_union(f.union)?;
            match stack.pop() {
                None => {
                    return Ok(ThompsonRef { start, end });
                }
                Some(mut parent) => {
                    // OK because the only way a frame gets pushed on to the
                    // stack (aside from the root) is when a transition has
                    // been added to 'sparse'.
                    parent.sparse.last_mut().unwrap().next = start;
                    f = parent;
                }
            }
        }
    }