fn split(&self) -> Option<(ScalarRange, ScalarRange)> {
        if self.start < 0xE000 && self.end > 0xD7FF {
            Some((
                ScalarRange { start: self.start, end: 0xD7FF },
                ScalarRange { start: 0xE000, end: self.end },
            ))
        } else {
            None
        }
    }