fn restore_already_parsed_fragment(&mut self, fragment: Option<String>) {
        if let Some(ref fragment) = fragment {
            assert!(self.fragment_start.is_none());
            self.fragment_start = Some(to_u32(self.serialization.len()).unwrap());
            self.serialization.push('#');
            self.serialization.push_str(fragment);
        }
    }