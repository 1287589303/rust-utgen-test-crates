pub fn set_username(&mut self, username: &str) -> Result<(), ()> {
        // has_host implies !cannot_be_a_base
        if !self.has_host() || self.host() == Some(Host::Domain("")) || self.scheme() == "file" {
            return Err(());
        }
        let username_start = self.scheme_end + 3;
        debug_assert!(self.slice(self.scheme_end..username_start) == "://");
        if self.slice(username_start..self.username_end) == username {
            return Ok(());
        }
        let after_username = self.slice(self.username_end..).to_owned();
        self.serialization.truncate(username_start as usize);
        self.serialization
            .extend(utf8_percent_encode(username, USERINFO));

        let mut removed_bytes = self.username_end;
        self.username_end = to_u32(self.serialization.len()).unwrap();
        let mut added_bytes = self.username_end;

        let new_username_is_empty = self.username_end == username_start;
        match (new_username_is_empty, after_username.chars().next()) {
            (true, Some('@')) => {
                removed_bytes += 1;
                self.serialization.push_str(&after_username[1..]);
            }
            (false, Some('@')) | (_, Some(':')) | (true, _) => {
                self.serialization.push_str(&after_username);
            }
            (false, _) => {
                added_bytes += 1;
                self.serialization.push('@');
                self.serialization.push_str(&after_username);
            }
        }

        let adjust = |index: &mut u32| {
            *index -= removed_bytes;
            *index += added_bytes;
        };
        adjust(&mut self.host_start);
        adjust(&mut self.host_end);
        adjust(&mut self.path_start);
        if let Some(ref mut index) = self.query_start {
            adjust(index)
        }
        if let Some(ref mut index) = self.fragment_start {
            adjust(index)
        }
        Ok(())
    }