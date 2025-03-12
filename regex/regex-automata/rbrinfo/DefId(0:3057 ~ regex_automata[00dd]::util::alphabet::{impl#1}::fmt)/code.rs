fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            UnitKind::U8(b) => write!(f, "{:?}", DebugByte(b)),
            UnitKind::EOI(_) => write!(f, "EOI"),
        }
    }