pub fn set_fragment(&mut self, fragment: Option<&str>) {
        // Remove any previous fragment
        if let Some(start) = self.fragment_start {
            debug_assert!(self.byte_at(start) == b'#');
            self.serialization.truncate(start as usize);
        }
        // Write the new one
        if let Some(input) = fragment {
            self.fragment_start = Some(to_u32(self.serialization.len()).unwrap());
            self.serialization.push('#');
            self.mutate(|parser| parser.parse_fragment(parser::Input::new_no_trim(input)))
        } else {
            self.fragment_start = None;
            self.strip_trailing_spaces_from_opaque_path();
        }
    }