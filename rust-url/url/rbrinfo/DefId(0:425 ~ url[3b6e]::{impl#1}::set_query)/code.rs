pub fn set_query(&mut self, query: Option<&str>) {
        let fragment = self.take_fragment();

        // Remove any previous query
        if let Some(start) = self.query_start.take() {
            debug_assert!(self.byte_at(start) == b'?');
            self.serialization.truncate(start as usize);
        }
        // Write the new query, if any
        if let Some(input) = query {
            self.query_start = Some(to_u32(self.serialization.len()).unwrap());
            self.serialization.push('?');
            let scheme_type = SchemeType::from(self.scheme());
            let scheme_end = self.scheme_end;
            self.mutate(|parser| {
                let vfn = parser.violation_fn;
                parser.parse_query(
                    scheme_type,
                    scheme_end,
                    parser::Input::new_trim_tab_and_newlines(input, vfn),
                )
            });
        } else {
            self.query_start = None;
            if fragment.is_none() {
                self.strip_trailing_spaces_from_opaque_path();
            }
        }

        self.restore_already_parsed_fragment(fragment);
    }