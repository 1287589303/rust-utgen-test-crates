fn fmt_flags(&mut self, ast: &ast::Flags) -> fmt::Result {
        use crate::ast::{Flag, FlagsItemKind};

        for item in &ast.items {
            match item.kind {
                FlagsItemKind::Negation => self.wtr.write_str("-"),
                FlagsItemKind::Flag(ref flag) => match *flag {
                    Flag::CaseInsensitive => self.wtr.write_str("i"),
                    Flag::MultiLine => self.wtr.write_str("m"),
                    Flag::DotMatchesNewLine => self.wtr.write_str("s"),
                    Flag::SwapGreed => self.wtr.write_str("U"),
                    Flag::Unicode => self.wtr.write_str("u"),
                    Flag::CRLF => self.wtr.write_str("R"),
                    Flag::IgnoreWhitespace => self.wtr.write_str("x"),
                },
            }?;
        }
        Ok(())
    }