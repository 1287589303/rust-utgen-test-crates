pub fn set_password(&mut self, password: Option<&str>) -> Result<(), ()> {
        // has_host implies !cannot_be_a_base
        if !self.has_host() || self.host() == Some(Host::Domain("")) || self.scheme() == "file" {
            return Err(());
        }
        let password = password.unwrap_or_default();
        if !password.is_empty() {
            let host_and_after = self.slice(self.host_start..).to_owned();
            self.serialization.truncate(self.username_end as usize);
            self.serialization.push(':');
            self.serialization
                .extend(utf8_percent_encode(password, USERINFO));
            self.serialization.push('@');

            let old_host_start = self.host_start;
            let new_host_start = to_u32(self.serialization.len()).unwrap();
            let adjust = |index: &mut u32| {
                *index -= old_host_start;
                *index += new_host_start;
            };
            self.host_start = new_host_start;
            adjust(&mut self.host_end);
            adjust(&mut self.path_start);
            if let Some(ref mut index) = self.query_start {
                adjust(index)
            }
            if let Some(ref mut index) = self.fragment_start {
                adjust(index)
            }

            self.serialization.push_str(&host_and_after);
        } else if self.byte_at(self.username_end) == b':' {
            // If there is a password to remove
            let has_username_or_password = self.byte_at(self.host_start - 1) == b'@';
            debug_assert!(has_username_or_password);
            let username_start = self.scheme_end + 3;
            let empty_username = username_start == self.username_end;
            let start = self.username_end; // Remove the ':'
            let end = if empty_username {
                self.host_start // Remove the '@' as well
            } else {
                self.host_start - 1 // Keep the '@' to separate the username from the host
            };
            self.serialization.drain(start as usize..end as usize);
            let offset = end - start;
            self.host_start -= offset;
            self.host_end -= offset;
            self.path_start -= offset;
            if let Some(ref mut index) = self.query_start {
                *index -= offset
            }
            if let Some(ref mut index) = self.fragment_start {
                *index -= offset
            }
        }
        Ok(())
    }