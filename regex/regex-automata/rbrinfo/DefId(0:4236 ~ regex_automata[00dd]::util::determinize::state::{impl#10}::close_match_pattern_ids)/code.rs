fn close_match_pattern_ids(&mut self) {
        // If we never wrote any pattern IDs, then there's nothing to do here.
        if !self.repr().has_pattern_ids() {
            return;
        }
        let patsize = PatternID::SIZE;
        let pattern_bytes = self.0.len() - 13;
        // Every pattern ID uses 4 bytes, so number of bytes should be
        // divisible by 4.
        assert_eq!(pattern_bytes % patsize, 0);
        // This unwrap is OK since we are guaranteed that the maximum number
        // of possible patterns fits into a u32.
        let count32 = u32::try_from(pattern_bytes / patsize).unwrap();
        wire::NE::write_u32(count32, &mut self.0[9..13]);
    }