pub fn set_host(&mut self, host: Option<&str>) -> Result<(), ParseError> {
        if self.cannot_be_a_base() {
            return Err(ParseError::SetHostOnCannotBeABaseUrl);
        }

        let scheme_type = SchemeType::from(self.scheme());

        if let Some(host) = host {
            if host.is_empty() && scheme_type.is_special() && !scheme_type.is_file() {
                return Err(ParseError::EmptyHost);
            }
            let mut host_substr = host;
            // Otherwise, if c is U+003A (:) and the [] flag is unset, then
            if !host.starts_with('[') || !host.ends_with(']') {
                match host.find(':') {
                    Some(0) => {
                        // If buffer is the empty string, validation error, return failure.
                        return Err(ParseError::InvalidDomainCharacter);
                    }
                    // Let host be the result of host parsing buffer
                    Some(colon_index) => {
                        host_substr = &host[..colon_index];
                    }
                    None => {}
                }
            }
            if SchemeType::from(self.scheme()).is_special() {
                self.set_host_internal(Host::parse(host_substr)?, None);
            } else {
                self.set_host_internal(Host::parse_opaque(host_substr)?, None);
            }
        } else if self.has_host() {
            if scheme_type.is_special() && !scheme_type.is_file() {
                return Err(ParseError::EmptyHost);
            } else if self.serialization.len() == self.path_start as usize {
                self.serialization.push('/');
            }
            debug_assert!(self.byte_at(self.scheme_end) == b':');
            debug_assert!(self.byte_at(self.path_start) == b'/');

            let new_path_start = if scheme_type.is_file() {
                self.scheme_end + 3
            } else {
                self.scheme_end + 1
            };

            self.serialization
                .drain(new_path_start as usize..self.path_start as usize);
            let offset = self.path_start - new_path_start;
            self.path_start = new_path_start;
            self.username_end = new_path_start;
            self.host_start = new_path_start;
            self.host_end = new_path_start;
            self.port = None;
            if let Some(ref mut index) = self.query_start {
                *index -= offset
            }
            if let Some(ref mut index) = self.fragment_start {
                *index -= offset
            }
        }
        Ok(())
    }