fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result {
        use crate::ast::GroupKind::*;
        match ast.kind {
            CaptureIndex(_) => self.wtr.write_str("("),
            CaptureName { ref name, starts_with_p } => {
                let start = if starts_with_p { "(?P<" } else { "(?<" };
                self.wtr.write_str(start)?;
                self.wtr.write_str(&name.name)?;
                self.wtr.write_str(">")?;
                Ok(())
            }
            NonCapturing(ref flags) => {
                self.wtr.write_str("(?")?;
                self.fmt_flags(flags)?;
                self.wtr.write_str(":")?;
                Ok(())
            }
        }
    }