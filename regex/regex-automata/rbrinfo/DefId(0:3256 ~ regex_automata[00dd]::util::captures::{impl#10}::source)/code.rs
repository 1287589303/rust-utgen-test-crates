fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            GroupInfoErrorKind::TooManyPatterns { .. }
            | GroupInfoErrorKind::TooManyGroups { .. }
            | GroupInfoErrorKind::MissingGroups { .. }
            | GroupInfoErrorKind::FirstMustBeUnnamed { .. }
            | GroupInfoErrorKind::Duplicate { .. } => None,
        }
    }