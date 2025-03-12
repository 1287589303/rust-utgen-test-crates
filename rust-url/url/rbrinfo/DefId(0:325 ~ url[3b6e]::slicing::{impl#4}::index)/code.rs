fn index(&self, position: Position) -> usize {
        match position {
            Position::BeforeScheme => 0,

            Position::AfterScheme => self.scheme_end as usize,

            Position::BeforeUsername => {
                if self.has_authority() {
                    self.scheme_end as usize + "://".len()
                } else {
                    debug_assert!(self.byte_at(self.scheme_end) == b':');
                    debug_assert!(self.scheme_end + ":".len() as u32 == self.username_end);
                    self.scheme_end as usize + ":".len()
                }
            }

            Position::AfterUsername => self.username_end as usize,

            Position::BeforePassword => {
                if self.has_authority() && self.byte_at(self.username_end) == b':' {
                    self.username_end as usize + ":".len()
                } else {
                    debug_assert!(self.username_end == self.host_start);
                    self.username_end as usize
                }
            }

            Position::AfterPassword => {
                if self.has_authority() && self.byte_at(self.username_end) == b':' {
                    debug_assert!(self.byte_at(self.host_start - "@".len() as u32) == b'@');
                    self.host_start as usize - "@".len()
                } else {
                    debug_assert!(self.username_end == self.host_start);
                    self.host_start as usize
                }
            }

            Position::BeforeHost => self.host_start as usize,

            Position::AfterHost => self.host_end as usize,

            Position::BeforePort => {
                if self.port.is_some() {
                    debug_assert!(self.byte_at(self.host_end) == b':');
                    self.host_end as usize + ":".len()
                } else {
                    self.host_end as usize
                }
            }

            Position::AfterPort => {
                if let Some(port) = self.port {
                    debug_assert!(self.byte_at(self.host_end) == b':');
                    self.host_end as usize + ":".len() + count_digits(port)
                } else {
                    self.host_end as usize
                }
            }

            Position::BeforePath => self.path_start as usize,

            Position::AfterPath => match (self.query_start, self.fragment_start) {
                (Some(q), _) => q as usize,
                (None, Some(f)) => f as usize,
                (None, None) => self.serialization.len(),
            },

            Position::BeforeQuery => match (self.query_start, self.fragment_start) {
                (Some(q), _) => {
                    debug_assert!(self.byte_at(q) == b'?');
                    q as usize + "?".len()
                }
                (None, Some(f)) => f as usize,
                (None, None) => self.serialization.len(),
            },

            Position::AfterQuery => match self.fragment_start {
                None => self.serialization.len(),
                Some(f) => f as usize,
            },

            Position::BeforeFragment => match self.fragment_start {
                Some(f) => {
                    debug_assert!(self.byte_at(f) == b'#');
                    f as usize + "#".len()
                }
                None => self.serialization.len(),
            },

            Position::AfterFragment => self.serialization.len(),
        }
    }