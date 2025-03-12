fn hir_anchor_end(&self) -> Hir {
        let look = if self.flags().multi_line {
            if self.flags().crlf {
                hir::Look::EndCRLF
            } else {
                hir::Look::EndLF
            }
        } else {
            hir::Look::End
        };
        Hir::look(look)
    }