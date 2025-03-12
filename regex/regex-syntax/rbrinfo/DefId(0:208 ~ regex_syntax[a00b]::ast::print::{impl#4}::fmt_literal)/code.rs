fn fmt_literal(&mut self, ast: &ast::Literal) -> fmt::Result {
        use crate::ast::LiteralKind::*;

        match ast.kind {
            Verbatim => self.wtr.write_char(ast.c),
            Meta | Superfluous => write!(self.wtr, r"\{}", ast.c),
            Octal => write!(self.wtr, r"\{:o}", u32::from(ast.c)),
            HexFixed(ast::HexLiteralKind::X) => {
                write!(self.wtr, r"\x{:02X}", u32::from(ast.c))
            }
            HexFixed(ast::HexLiteralKind::UnicodeShort) => {
                write!(self.wtr, r"\u{:04X}", u32::from(ast.c))
            }
            HexFixed(ast::HexLiteralKind::UnicodeLong) => {
                write!(self.wtr, r"\U{:08X}", u32::from(ast.c))
            }
            HexBrace(ast::HexLiteralKind::X) => {
                write!(self.wtr, r"\x{{{:X}}}", u32::from(ast.c))
            }
            HexBrace(ast::HexLiteralKind::UnicodeShort) => {
                write!(self.wtr, r"\u{{{:X}}}", u32::from(ast.c))
            }
            HexBrace(ast::HexLiteralKind::UnicodeLong) => {
                write!(self.wtr, r"\U{{{:X}}}", u32::from(ast.c))
            }
            Special(ast::SpecialLiteralKind::Bell) => {
                self.wtr.write_str(r"\a")
            }
            Special(ast::SpecialLiteralKind::FormFeed) => {
                self.wtr.write_str(r"\f")
            }
            Special(ast::SpecialLiteralKind::Tab) => self.wtr.write_str(r"\t"),
            Special(ast::SpecialLiteralKind::LineFeed) => {
                self.wtr.write_str(r"\n")
            }
            Special(ast::SpecialLiteralKind::CarriageReturn) => {
                self.wtr.write_str(r"\r")
            }
            Special(ast::SpecialLiteralKind::VerticalTab) => {
                self.wtr.write_str(r"\v")
            }
            Special(ast::SpecialLiteralKind::Space) => {
                self.wtr.write_str(r"\ ")
            }
        }
    }