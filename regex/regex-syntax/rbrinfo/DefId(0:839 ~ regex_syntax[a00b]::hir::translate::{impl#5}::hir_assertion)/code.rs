fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {
        let unicode = self.flags().unicode();
        let multi_line = self.flags().multi_line();
        let crlf = self.flags().crlf();
        Ok(match asst.kind {
            ast::AssertionKind::StartLine => Hir::look(if multi_line {
                if crlf {
                    hir::Look::StartCRLF
                } else {
                    hir::Look::StartLF
                }
            } else {
                hir::Look::Start
            }),
            ast::AssertionKind::EndLine => Hir::look(if multi_line {
                if crlf {
                    hir::Look::EndCRLF
                } else {
                    hir::Look::EndLF
                }
            } else {
                hir::Look::End
            }),
            ast::AssertionKind::StartText => Hir::look(hir::Look::Start),
            ast::AssertionKind::EndText => Hir::look(hir::Look::End),
            ast::AssertionKind::WordBoundary => Hir::look(if unicode {
                hir::Look::WordUnicode
            } else {
                hir::Look::WordAscii
            }),
            ast::AssertionKind::NotWordBoundary => Hir::look(if unicode {
                hir::Look::WordUnicodeNegate
            } else {
                hir::Look::WordAsciiNegate
            }),
            ast::AssertionKind::WordBoundaryStart
            | ast::AssertionKind::WordBoundaryStartAngle => {
                Hir::look(if unicode {
                    hir::Look::WordStartUnicode
                } else {
                    hir::Look::WordStartAscii
                })
            }
            ast::AssertionKind::WordBoundaryEnd
            | ast::AssertionKind::WordBoundaryEndAngle => {
                Hir::look(if unicode {
                    hir::Look::WordEndUnicode
                } else {
                    hir::Look::WordEndAscii
                })
            }
            ast::AssertionKind::WordBoundaryStartHalf => {
                Hir::look(if unicode {
                    hir::Look::WordStartHalfUnicode
                } else {
                    hir::Look::WordStartHalfAscii
                })
            }
            ast::AssertionKind::WordBoundaryEndHalf => Hir::look(if unicode {
                hir::Look::WordEndHalfUnicode
            } else {
                hir::Look::WordEndHalfAscii
            }),
        })
    }