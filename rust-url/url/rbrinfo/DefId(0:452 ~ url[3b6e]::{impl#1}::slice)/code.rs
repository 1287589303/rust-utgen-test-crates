fn slice<R>(&self, range: R) -> &str
    where
        R: RangeArg,
    {
        range.slice_of(&self.serialization)
    }