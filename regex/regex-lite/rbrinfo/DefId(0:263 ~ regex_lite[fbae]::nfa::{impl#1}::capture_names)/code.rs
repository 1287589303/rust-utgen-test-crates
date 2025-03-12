pub(crate) fn capture_names(&self) -> CaptureNames<'_> {
        CaptureNames { it: self.cap_index_to_name.iter() }
    }