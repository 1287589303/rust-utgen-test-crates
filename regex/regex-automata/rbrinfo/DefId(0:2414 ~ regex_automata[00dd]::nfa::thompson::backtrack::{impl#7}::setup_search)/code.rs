fn setup_search(
        &mut self,
        re: &BoundedBacktracker,
        input: &Input<'_>,
    ) -> Result<(), MatchError> {
        self.stack.clear();
        self.visited.setup_search(re, input)?;
        Ok(())
    }