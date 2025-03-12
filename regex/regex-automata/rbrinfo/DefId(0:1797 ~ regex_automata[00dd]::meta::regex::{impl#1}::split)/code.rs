pub fn split<'r, 'h, I: Into<Input<'h>>>(
        &'r self,
        input: I,
    ) -> Split<'r, 'h> {
        Split { finder: self.find_iter(input), last: 0 }
    }