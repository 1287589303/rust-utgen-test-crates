pub fn add_capture_start(
        &mut self,
        next: StateID,
        group_index: u32,
        name: Option<Arc<str>>,
    ) -> Result<StateID, BuildError> {
        let pid = self.current_pattern_id();
        let group_index = match SmallIndex::try_from(group_index) {
            Err(_) => {
                return Err(BuildError::invalid_capture_index(group_index))
            }
            Ok(group_index) => group_index,
        };
        // Make sure we have space to insert our (pid,index)|-->name mapping.
        if pid.as_usize() >= self.captures.len() {
            for _ in 0..=(pid.as_usize() - self.captures.len()) {
                self.captures.push(vec![]);
            }
        }
        // In the case where 'group_index < self.captures[pid].len()', it means
        // that we are adding a duplicate capture group. This is somewhat
        // weird, but permissible because the capture group itself can be
        // repeated in the syntax. For example, '([a-z]){4}' will produce 4
        // capture groups. In practice, only the last will be set at search
        // time when a match occurs. For duplicates, we don't need to push
        // anything other than a CaptureStart NFA state.
        if group_index.as_usize() >= self.captures[pid].len() {
            // For discontiguous indices, push placeholders for earlier capture
            // groups that weren't explicitly added.
            for _ in 0..(group_index.as_usize() - self.captures[pid].len()) {
                self.captures[pid].push(None);
            }
            self.captures[pid].push(name);
        }
        self.add(State::CaptureStart { pattern_id: pid, group_index, next })
    }