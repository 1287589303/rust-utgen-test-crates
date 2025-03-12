fn from(e: InvalidBase64Details) -> Self {
        DecodeError::InvalidBase64(InvalidBase64(e))
    }