fn next(&mut self) -> Option<(PatternID, usize, Option<&'a str>)> {
        // If the group info has no captures, then we never have anything
        // to yield. We need to consider this case explicitly (at time of
        // writing) because 'pattern_capture_names' will panic if captures
        // aren't enabled.
        if self.group_info.0.index_to_name.is_empty() {
            return None;
        }
        if self.current_pid.is_none() {
            self.current_pid = Some(self.pids.next()?);
        }
        let pid = self.current_pid.unwrap();
        if self.names.is_none() {
            self.names = Some(self.group_info.pattern_names(pid).enumerate());
        }
        let (group_index, name) = match self.names.as_mut().unwrap().next() {
            Some((group_index, name)) => (group_index, name),
            None => {
                self.current_pid = None;
                self.names = None;
                return self.next();
            }
        };
        Some((pid, group_index, name))
    }