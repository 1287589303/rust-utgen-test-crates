fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBase64(inner) => write!(f, "base64 not valid: {}", inner),
            Self::WriteError(err) => write!(f, "write error: {}", err),
        }
    }