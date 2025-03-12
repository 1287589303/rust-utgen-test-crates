pub fn byte(&self) -> Option<u8> {
        match self.kind {
            LiteralKind::HexFixed(HexLiteralKind::X) => {
                u8::try_from(self.c).ok()
            }
            _ => None,
        }
    }