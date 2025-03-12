fn slice_of<'a>(&self, s: &'a str) -> &'a str {
        &s[self.start as usize..self.end as usize]
    }