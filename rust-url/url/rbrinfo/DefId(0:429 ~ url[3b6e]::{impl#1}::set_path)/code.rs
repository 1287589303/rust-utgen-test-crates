pub fn set_path(&mut self, mut path: &str) {
        let after_path = self.take_after_path();
        let old_after_path_pos = to_u32(self.serialization.len()).unwrap();
        let cannot_be_a_base = self.cannot_be_a_base();
        let scheme_type = SchemeType::from(self.scheme());
        self.serialization.truncate(self.path_start as usize);
        self.mutate(|parser| {
            if cannot_be_a_base {
                if path.starts_with('/') {
                    parser.serialization.push_str("%2F");
                    path = &path[1..];
                }
                parser.parse_cannot_be_a_base_path(parser::Input::new_no_trim(path));
            } else {
                let mut has_host = true; // FIXME
                parser.parse_path_start(
                    scheme_type,
                    &mut has_host,
                    parser::Input::new_no_trim(path),
                );
            }
        });
        self.restore_after_path(old_after_path_pos, &after_path);
    }