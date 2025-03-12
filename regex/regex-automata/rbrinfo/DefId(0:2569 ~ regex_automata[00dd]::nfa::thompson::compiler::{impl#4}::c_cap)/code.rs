fn c_cap(
        &self,
        index: u32,
        name: Option<&str>,
        expr: &Hir,
    ) -> Result<ThompsonRef, BuildError> {
        match self.config.get_which_captures() {
            // No capture states means we always skip them.
            WhichCaptures::None => return self.c(expr),
            // Implicit captures states means we only add when index==0 since
            // index==0 implies the group is implicit.
            WhichCaptures::Implicit if index > 0 => return self.c(expr),
            _ => {}
        }

        let start = self.add_capture_start(index, name)?;
        let inner = self.c(expr)?;
        let end = self.add_capture_end(index)?;
        self.patch(start, inner.start)?;
        self.patch(inner.end, end)?;
        Ok(ThompsonRef { start, end })
    }