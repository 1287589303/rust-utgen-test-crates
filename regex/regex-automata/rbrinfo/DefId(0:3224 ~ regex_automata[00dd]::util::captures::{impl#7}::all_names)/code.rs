pub fn all_names(&self) -> GroupInfoAllNames<'_> {
        GroupInfoAllNames {
            group_info: self,
            pids: PatternID::iter(self.pattern_len()),
            current_pid: None,
            names: None,
        }
    }