pub fn set_scheme(&mut self, scheme: &str) -> Result<(), ()> {
        let mut parser = Parser::for_setter(String::new());
        let remaining = parser.parse_scheme(parser::Input::new_no_trim(scheme))?;
        let new_scheme_type = SchemeType::from(&parser.serialization);
        let old_scheme_type = SchemeType::from(self.scheme());
        // If url’s scheme is a special scheme and buffer is not a special scheme, then return.
        if (new_scheme_type.is_special() && !old_scheme_type.is_special()) ||
            // If url’s scheme is not a special scheme and buffer is a special scheme, then return.
            (!new_scheme_type.is_special() && old_scheme_type.is_special()) ||
            // If url includes credentials or has a non-null port, and buffer is "file", then return.
            // If url’s scheme is "file" and its host is an empty host or null, then return.
            (new_scheme_type.is_file() && self.has_authority())
        {
            return Err(());
        }

        if !remaining.is_empty() || (!self.has_host() && new_scheme_type.is_special()) {
            return Err(());
        }
        let old_scheme_end = self.scheme_end;
        let new_scheme_end = to_u32(parser.serialization.len()).unwrap();
        let adjust = |index: &mut u32| {
            *index -= old_scheme_end;
            *index += new_scheme_end;
        };

        self.scheme_end = new_scheme_end;
        adjust(&mut self.username_end);
        adjust(&mut self.host_start);
        adjust(&mut self.host_end);
        adjust(&mut self.path_start);
        if let Some(ref mut index) = self.query_start {
            adjust(index)
        }
        if let Some(ref mut index) = self.fragment_start {
            adjust(index)
        }

        parser.serialization.push_str(self.slice(old_scheme_end..));
        self.serialization = parser.serialization;

        // Update the port so it can be removed
        // If it is the scheme's default
        // we don't mind it silently failing
        // if there was no port in the first place
        let previous_port = self.port();
        let _ = self.set_port(previous_port);

        Ok(())
    }