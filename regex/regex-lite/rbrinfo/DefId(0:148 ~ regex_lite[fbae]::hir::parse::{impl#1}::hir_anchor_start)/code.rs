fn hir_anchor_start(&self) -> Hir {
        let look = if self.flags().multi_line {
            if self.flags().crlf {
                hir::Look::StartCRLF
            } else {
                hir::Look::StartLF
            }
        } else {
            hir::Look::Start
        };
        Hir::look(look)
    }