fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.range.start == self.range.end {
            write!(
                f,
                "{:02X} => {:02X}",
                self.range.start,
                self.next_id.as_usize(),
            )
        } else {
            write!(
                f,
                "{:02X}-{:02X} => {:02X}",
                self.range.start,
                self.range.end,
                self.next_id.as_usize(),
            )
        }
    }