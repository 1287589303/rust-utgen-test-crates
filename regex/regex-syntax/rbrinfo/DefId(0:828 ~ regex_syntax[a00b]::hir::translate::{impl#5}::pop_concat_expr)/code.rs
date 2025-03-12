fn pop_concat_expr(&self) -> Option<Hir> {
        let frame = self.pop()?;
        match frame {
            HirFrame::Concat => None,
            HirFrame::Expr(expr) => Some(expr),
            HirFrame::Literal(lit) => Some(Hir::literal(lit)),
            HirFrame::ClassUnicode(_) => {
                unreachable!("expected expr or concat, got Unicode class")
            }
            HirFrame::ClassBytes(_) => {
                unreachable!("expected expr or concat, got byte class")
            }
            HirFrame::Repetition => {
                unreachable!("expected expr or concat, got repetition")
            }
            HirFrame::Group { .. } => {
                unreachable!("expected expr or concat, got group")
            }
            HirFrame::Alternation => {
                unreachable!("expected expr or concat, got alt marker")
            }
            HirFrame::AlternationBranch => {
                unreachable!("expected expr or concat, got alt branch marker")
            }
        }
    }