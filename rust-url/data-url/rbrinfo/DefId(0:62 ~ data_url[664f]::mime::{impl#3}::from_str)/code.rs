fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).ok_or(MimeParsingError(()))
    }