pub fn iter(&self) -> CapturesPatternIter<'_> {
        let names = self
            .pattern()
            .map_or(GroupInfoPatternNames::empty().enumerate(), |pid| {
                self.group_info().pattern_names(pid).enumerate()
            });
        CapturesPatternIter { caps: self, names }
    }