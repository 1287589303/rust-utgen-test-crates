fn hir_char(&self, ch: char) -> Hir {
        if self.flags().case_insensitive {
            let this = hir::ClassRange { start: ch, end: ch };
            if let Some(folded) = this.ascii_case_fold() {
                return Hir::class(hir::Class::new([this, folded]));
            }
        }
        Hir::char(ch)
    }