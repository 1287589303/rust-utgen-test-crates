fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::error::Formatter::from(self).fmt(f)
    }