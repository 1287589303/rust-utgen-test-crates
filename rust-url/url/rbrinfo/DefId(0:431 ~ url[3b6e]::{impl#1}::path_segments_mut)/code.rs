pub fn path_segments_mut(&mut self) -> Result<PathSegmentsMut<'_>, ()> {
        if self.cannot_be_a_base() {
            Err(())
        } else {
            Ok(path_segments::new(self))
        }
    }