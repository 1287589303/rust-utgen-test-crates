fn has_appropriately_joining_char<I: Iterator<Item = char>>(
        &self,
        iter: I,
        required_mask: JoiningTypeMask,
    ) -> bool {
        for c in iter {
            let jt = self.data.joining_type(c);
            if jt.to_mask().intersects(required_mask) {
                return true;
            }
            if jt.is_transparent() {
                continue;
            }
            return false;
        }
        false
    }