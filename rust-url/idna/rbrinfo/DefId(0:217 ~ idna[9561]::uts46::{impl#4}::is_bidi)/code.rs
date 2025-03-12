fn is_bidi(&self, buffer: &[char]) -> bool {
        for &c in buffer {
            if c < '\u{0590}' {
                // Below Hebrew
                continue;
            }
            if in_inclusive_range_char(c, '\u{0900}', '\u{FB1C}') {
                debug_assert_ne!(c, '\u{200F}'); // disallowed
                continue;
            }
            if in_inclusive_range_char(c, '\u{1F000}', '\u{3FFFF}') {
                continue;
            }
            if in_inclusive_range_char(c, '\u{FF00}', '\u{107FF}') {
                continue;
            }
            if in_inclusive_range_char(c, '\u{11000}', '\u{1E7FF}') {
                continue;
            }
            if RTL_MASK.intersects(self.data.bidi_class(c).to_mask()) {
                return true;
            }
        }
        false
    }