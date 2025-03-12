pub fn build_many_sparse<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError> {
        let re = self.build_many(patterns)?;
        let forward = re.forward().to_sparse()?;
        let reverse = re.reverse().to_sparse()?;
        Ok(self.build_from_dfas(forward, reverse))
    }