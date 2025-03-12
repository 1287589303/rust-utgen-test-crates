fn c_capture(
        &self,
        index: u32,
        name: Option<&str>,
        hir: &Hir,
    ) -> Result<ThompsonRef, Error> {
        // For discontiguous indices, push placeholders for earlier capture
        // groups that weren't explicitly added. This can happen, for example,
        // with patterns like '(a){0}(a)' where '(a){0}' is completely removed
        // from the pattern.
        let existing_groups_len = self.nfa.borrow().cap_index_to_name.len();
        for _ in 0..(index.as_usize().saturating_sub(existing_groups_len)) {
            self.nfa.borrow_mut().cap_index_to_name.push(None);
        }
        if index.as_usize() >= existing_groups_len {
            if let Some(name) = name {
                let name = Arc::from(name);
                let mut nfa = self.nfa.borrow_mut();
                nfa.cap_name_to_index.insert(Arc::clone(&name), index);
                nfa.cap_index_to_name.push(Some(Arc::clone(&name)));
                // This is an approximation.
                nfa.memory_extra += name.len() + size_of::<u32>();
            } else {
                self.nfa.borrow_mut().cap_index_to_name.push(None);
            }
        }

        let Some(slot) = index.checked_mul(2) else {
            return Err(Error::new("capture group slots exhausted"));
        };
        let start = self.add(State::Capture { target: 0, slot })?;
        let inner = self.c(hir)?;
        let Some(slot) = slot.checked_add(1) else {
            return Err(Error::new("capture group slots exhausted"));
        };
        let end = self.add(State::Capture { target: 0, slot })?;
        self.patch(start, inner.start)?;
        self.patch(inner.end, end)?;

        Ok(ThompsonRef { start, end })
    }