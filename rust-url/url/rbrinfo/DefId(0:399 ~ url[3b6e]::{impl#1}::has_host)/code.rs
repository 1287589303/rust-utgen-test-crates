pub fn has_host(&self) -> bool {
        !matches!(self.host, HostInternal::None)
    }