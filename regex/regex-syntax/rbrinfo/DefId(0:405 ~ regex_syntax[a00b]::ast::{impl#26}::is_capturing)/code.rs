pub fn is_capturing(&self) -> bool {
        match self.kind {
            GroupKind::CaptureIndex(_) | GroupKind::CaptureName { .. } => true,
            GroupKind::NonCapturing(_) => false,
        }
    }