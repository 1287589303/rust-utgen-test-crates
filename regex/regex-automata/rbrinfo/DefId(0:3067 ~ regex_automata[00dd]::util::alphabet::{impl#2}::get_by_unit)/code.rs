pub fn get_by_unit(&self, unit: Unit) -> usize {
        match unit.0 {
            UnitKind::U8(b) => usize::from(self.get(b)),
            UnitKind::EOI(b) => usize::from(b),
        }
    }