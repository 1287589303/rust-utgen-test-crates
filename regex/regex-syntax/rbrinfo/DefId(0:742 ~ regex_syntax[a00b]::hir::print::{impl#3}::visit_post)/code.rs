fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
        match *hir.kind() {
            // Handled during visit_pre
            HirKind::Empty
            | HirKind::Literal(_)
            | HirKind::Class(_)
            | HirKind::Look(_) => {}
            HirKind::Repetition(ref x) => {
                match (x.min, x.max) {
                    (0, Some(1)) => {
                        self.wtr.write_str("?")?;
                    }
                    (0, None) => {
                        self.wtr.write_str("*")?;
                    }
                    (1, None) => {
                        self.wtr.write_str("+")?;
                    }
                    (1, Some(1)) => {
                        // 'a{1}' and 'a{1}?' are exactly equivalent to 'a'.
                        return Ok(());
                    }
                    (m, None) => {
                        write!(self.wtr, "{{{},}}", m)?;
                    }
                    (m, Some(n)) if m == n => {
                        write!(self.wtr, "{{{}}}", m)?;
                        // a{m} and a{m}? are always exactly equivalent.
                        return Ok(());
                    }
                    (m, Some(n)) => {
                        write!(self.wtr, "{{{},{}}}", m, n)?;
                    }
                }
                if !x.greedy {
                    self.wtr.write_str("?")?;
                }
            }
            HirKind::Capture(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => {
                self.wtr.write_str(r")")?;
            }
        }
        Ok(())
    }