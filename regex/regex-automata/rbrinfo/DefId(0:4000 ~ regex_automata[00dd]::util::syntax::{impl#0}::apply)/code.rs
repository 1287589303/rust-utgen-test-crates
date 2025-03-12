pub(crate) fn apply(&self, builder: &mut ParserBuilder) {
        builder
            .unicode(self.unicode)
            .case_insensitive(self.case_insensitive)
            .multi_line(self.multi_line)
            .dot_matches_new_line(self.dot_matches_new_line)
            .crlf(self.crlf)
            .line_terminator(self.line_terminator)
            .swap_greed(self.swap_greed)
            .ignore_whitespace(self.ignore_whitespace)
            .utf8(self.utf8)
            .nest_limit(self.nest_limit)
            .octal(self.octal);
    }