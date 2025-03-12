pub fn build(&self) -> Result<RegexSet, Error> {
            self.builder.build_many_bytes()
        }