fn pop_alt_expr(&self) -> Option<Hir> {
        let frame = self.pop()?;
        match frame {
            HirFrame::Alternation => None,
            HirFrame::Expr(expr) => Some(expr),
            HirFrame::Literal(lit) => Some(Hir::literal(lit)),
            HirFrame::ClassUnicode(_) => {
                unreachable!("expected expr or alt, got Unicode class")
            }
            HirFrame::ClassBytes(_) => {
                unreachable!("expected expr or alt, got byte class")
            }
            HirFrame::Repetition => {
                unreachable!("expected expr or alt, got repetition")
            }
            HirFrame::Group { .. } => {
                unreachable!("expected expr or alt, got group")
            }
            HirFrame::Concat => {
                unreachable!("expected expr or alt, got concat marker")
            }
            HirFrame::AlternationBranch => {
                unreachable!("expected expr or alt, got alt branch marker")
            }
        }
    }