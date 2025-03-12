// Answer 0

#[test]
fn test_is_word_byte_with_all_u8_values() {
    for byte in 0u8..=255 {
        let unit = Unit(UnitKind::U8(byte));
        unit.is_word_byte();
    }
}

#[test]
fn test_is_word_byte_with_eoi_values() {
    for num_byte_equiv_classes in 0..=100 {
        let unit = Unit(UnitKind::EOI(num_byte_equiv_classes));
        unit.is_word_byte();
    }
}

