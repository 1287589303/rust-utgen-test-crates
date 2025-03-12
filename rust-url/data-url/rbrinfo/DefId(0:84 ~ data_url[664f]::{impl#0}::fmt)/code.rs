fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotADataUrl => write!(f, "not a valid data url"),
            Self::NoComma => write!(
                f,
                "data url is missing comma delimiting attributes and body"
            ),
        }
    }