fn visit_pre(&mut self, ast: &Ast) -> Result<()> {
        match *ast {
            Ast::ClassBracketed(_) => {
                if self.flags().unicode() {
                    let cls = hir::ClassUnicode::empty();
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let cls = hir::ClassBytes::empty();
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            Ast::Repetition(_) => self.push(HirFrame::Repetition),
            Ast::Group(ref x) => {
                let old_flags = x
                    .flags()
                    .map(|ast| self.set_flags(ast))
                    .unwrap_or_else(|| self.flags());
                self.push(HirFrame::Group { old_flags });
            }
            Ast::Concat(_) => {
                self.push(HirFrame::Concat);
            }
            Ast::Alternation(ref x) => {
                self.push(HirFrame::Alternation);
                if !x.asts.is_empty() {
                    self.push(HirFrame::AlternationBranch);
                }
            }
            _ => {}
        }
        Ok(())
    }