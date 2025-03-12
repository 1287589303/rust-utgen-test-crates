fn parse(&self, tag: &str) -> Result<Cursor<'a>, Reject> {
        if self.starts_with(tag) {
            Ok(self.advance(tag.len()))
        } else {
            Err(Reject)
        }
    }