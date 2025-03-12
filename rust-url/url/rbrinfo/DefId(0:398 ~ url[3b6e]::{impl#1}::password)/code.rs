pub fn password(&self) -> Option<&str> {
        // This ':' is not the one marking a port number since a host can not be empty.
        // (Except for file: URLs, which do not have port numbers.)
        if self.has_authority()
            && self.username_end != self.serialization.len() as u32
            && self.byte_at(self.username_end) == b':'
        {
            debug_assert!(self.byte_at(self.host_start - 1) == b'@');
            Some(self.slice(self.username_end + 1..self.host_start - 1))
        } else {
            None
        }
    }