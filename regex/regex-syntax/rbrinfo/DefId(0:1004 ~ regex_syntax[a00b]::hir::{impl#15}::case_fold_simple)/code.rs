fn case_fold_simple(
        &self,
        ranges: &mut Vec<ClassUnicodeRange>,
    ) -> Result<(), unicode::CaseFoldError> {
        let mut folder = unicode::SimpleCaseFolder::new()?;
        if !folder.overlaps(self.start, self.end) {
            return Ok(());
        }
        let (start, end) = (u32::from(self.start), u32::from(self.end));
        for cp in (start..=end).filter_map(char::from_u32) {
            for &cp_folded in folder.mapping(cp) {
                ranges.push(ClassUnicodeRange::new(cp_folded, cp_folded));
            }
        }
        Ok(())
    }