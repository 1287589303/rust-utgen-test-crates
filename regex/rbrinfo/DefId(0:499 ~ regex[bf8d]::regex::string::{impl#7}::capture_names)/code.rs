pub fn capture_names(&self) -> CaptureNames<'_> {
        CaptureNames(self.meta.group_info().pattern_names(PatternID::ZERO))
    }