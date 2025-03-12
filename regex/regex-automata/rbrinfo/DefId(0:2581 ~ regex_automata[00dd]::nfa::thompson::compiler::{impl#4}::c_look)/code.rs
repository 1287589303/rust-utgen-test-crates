fn c_look(&self, anchor: &hir::Look) -> Result<ThompsonRef, BuildError> {
        let look = match *anchor {
            hir::Look::Start => Look::Start,
            hir::Look::End => Look::End,
            hir::Look::StartLF => Look::StartLF,
            hir::Look::EndLF => Look::EndLF,
            hir::Look::StartCRLF => Look::StartCRLF,
            hir::Look::EndCRLF => Look::EndCRLF,
            hir::Look::WordAscii => Look::WordAscii,
            hir::Look::WordAsciiNegate => Look::WordAsciiNegate,
            hir::Look::WordUnicode => Look::WordUnicode,
            hir::Look::WordUnicodeNegate => Look::WordUnicodeNegate,
            hir::Look::WordStartAscii => Look::WordStartAscii,
            hir::Look::WordEndAscii => Look::WordEndAscii,
            hir::Look::WordStartUnicode => Look::WordStartUnicode,
            hir::Look::WordEndUnicode => Look::WordEndUnicode,
            hir::Look::WordStartHalfAscii => Look::WordStartHalfAscii,
            hir::Look::WordEndHalfAscii => Look::WordEndHalfAscii,
            hir::Look::WordStartHalfUnicode => Look::WordStartHalfUnicode,
            hir::Look::WordEndHalfUnicode => Look::WordEndHalfUnicode,
        };
        let id = self.add_look(look)?;
        Ok(ThompsonRef { start: id, end: id })
    }