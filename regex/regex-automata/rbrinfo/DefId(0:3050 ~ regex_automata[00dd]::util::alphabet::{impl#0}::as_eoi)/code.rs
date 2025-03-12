pub fn as_eoi(self) -> Option<u16> {
        match self.0 {
            UnitKind::U8(_) => None,
            UnitKind::EOI(sentinel) => Some(sentinel),
        }
    }