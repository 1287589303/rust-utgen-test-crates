pub fn get_starts(&self) -> StartKind {
        self.start_kind.unwrap_or(StartKind::Both)
    }