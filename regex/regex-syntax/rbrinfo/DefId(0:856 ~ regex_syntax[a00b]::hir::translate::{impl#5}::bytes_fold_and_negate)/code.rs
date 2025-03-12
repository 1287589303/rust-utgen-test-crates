fn bytes_fold_and_negate(
        &self,
        span: &Span,
        negated: bool,
        class: &mut hir::ClassBytes,
    ) -> Result<()> {
        // Note that we must apply case folding before negation!
        // Consider `(?i)[^x]`. If we applied negation first, then
        // the result would be the character class that matched any
        // Unicode scalar value.
        if self.flags().case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
        if self.trans().utf8 && !class.is_ascii() {
            return Err(self.error(span.clone(), ErrorKind::InvalidUtf8));
        }
        Ok(())
    }