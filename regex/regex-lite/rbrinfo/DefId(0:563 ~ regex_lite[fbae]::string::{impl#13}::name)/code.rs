pub fn name(&self, name: &str) -> Option<Match<'h>> {
        let i = self.pikevm.nfa().to_index(name)?;
        self.get(i)
    }