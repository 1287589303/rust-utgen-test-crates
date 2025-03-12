fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            StartError::Cache { ref err } => Some(err),
            _ => None,
        }
    }