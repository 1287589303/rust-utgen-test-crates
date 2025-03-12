pub fn splitn<'r, 'h, I: Into<Input<'h>>>(
        &'r self,
        input: I,
        limit: usize,
    ) -> SplitN<'r, 'h> {
        SplitN { splits: self.split(input), limit }
    }