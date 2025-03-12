fn strip_trailing_spaces_from_opaque_path(&mut self) {
        if !self.cannot_be_a_base() {
            return;
        }

        if self.fragment_start.is_some() {
            return;
        }

        if self.query_start.is_some() {
            return;
        }

        let trailing_space_count = self
            .serialization
            .chars()
            .rev()
            .take_while(|c| *c == ' ')
            .count();

        let start = self.serialization.len() - trailing_space_count;

        self.serialization.truncate(start);
    }