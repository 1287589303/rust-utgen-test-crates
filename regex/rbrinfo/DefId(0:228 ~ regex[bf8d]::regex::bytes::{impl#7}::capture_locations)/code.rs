pub fn capture_locations(&self) -> CaptureLocations {
        CaptureLocations(self.meta.create_captures())
    }