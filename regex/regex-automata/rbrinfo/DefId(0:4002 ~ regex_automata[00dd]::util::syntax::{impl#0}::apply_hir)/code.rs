pub(crate) fn apply_hir(
        &self,
        builder: &mut hir::translate::TranslatorBuilder,
    ) {
        builder
            .unicode(self.unicode)
            .case_insensitive(self.case_insensitive)
            .multi_line(self.multi_line)
            .crlf(self.crlf)
            .dot_matches_new_line(self.dot_matches_new_line)
            .line_terminator(self.line_terminator)
            .swap_greed(self.swap_greed)
            .utf8(self.utf8);
    }