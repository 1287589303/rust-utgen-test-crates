fn fmt_assertion(&mut self, ast: &ast::Assertion) -> fmt::Result {
        use crate::ast::AssertionKind::*;
        match ast.kind {
            StartLine => self.wtr.write_str("^"),
            EndLine => self.wtr.write_str("$"),
            StartText => self.wtr.write_str(r"\A"),
            EndText => self.wtr.write_str(r"\z"),
            WordBoundary => self.wtr.write_str(r"\b"),
            NotWordBoundary => self.wtr.write_str(r"\B"),
            WordBoundaryStart => self.wtr.write_str(r"\b{start}"),
            WordBoundaryEnd => self.wtr.write_str(r"\b{end}"),
            WordBoundaryStartAngle => self.wtr.write_str(r"\<"),
            WordBoundaryEndAngle => self.wtr.write_str(r"\>"),
            WordBoundaryStartHalf => self.wtr.write_str(r"\b{start-half}"),
            WordBoundaryEndHalf => self.wtr.write_str(r"\b{end-half}"),
        }
    }