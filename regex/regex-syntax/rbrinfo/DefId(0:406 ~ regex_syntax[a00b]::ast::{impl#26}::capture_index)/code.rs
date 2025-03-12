pub fn capture_index(&self) -> Option<u32> {
        match self.kind {
            GroupKind::CaptureIndex(i) => Some(i),
            GroupKind::CaptureName { ref name, .. } => Some(name.index),
            GroupKind::NonCapturing(_) => None,
        }
    }