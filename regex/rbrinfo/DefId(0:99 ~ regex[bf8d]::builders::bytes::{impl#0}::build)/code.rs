pub fn build(&self) -> Result<Regex, Error> {
            self.builder.build_one_bytes()
        }