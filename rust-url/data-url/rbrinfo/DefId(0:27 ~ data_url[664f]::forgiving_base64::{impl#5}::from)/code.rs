fn from(e: DecodeError<Impossible>) -> Self {
        match e {
            DecodeError::InvalidBase64(e) => e,
            DecodeError::WriteError(e) => match e {},
        }
    }