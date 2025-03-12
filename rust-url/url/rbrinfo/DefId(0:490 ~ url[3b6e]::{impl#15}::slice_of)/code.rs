fn slice_of<'a>(&self, s: &'a str) -> &'a str {
        &s[..self.end as usize]
    }