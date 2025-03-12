fn visit_alternation_in(&mut self) -> Result<()> {
        self.push(HirFrame::AlternationBranch);
        Ok(())
    }