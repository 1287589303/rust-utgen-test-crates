pub fn eoi(num_byte_equiv_classes: usize) -> Unit {
        assert!(
            num_byte_equiv_classes <= 256,
            "max number of byte-based equivalent classes is 256, but got {}",
            num_byte_equiv_classes,
        );
        Unit(UnitKind::EOI(u16::try_from(num_byte_equiv_classes).unwrap()))
    }