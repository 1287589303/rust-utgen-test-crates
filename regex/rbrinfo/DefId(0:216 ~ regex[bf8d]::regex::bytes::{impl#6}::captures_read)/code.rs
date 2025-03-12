pub fn captures_read<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h [u8],
    ) -> Option<Match<'h>> {
        self.captures_read_at(locs, haystack, 0)
    }