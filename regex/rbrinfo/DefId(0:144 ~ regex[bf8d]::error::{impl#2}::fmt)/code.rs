fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Error::Syntax(ref err) => err.fmt(f),
            Error::CompiledTooBig(limit) => write!(
                f,
                "Compiled regex exceeds size limit of {} bytes.",
                limit
            ),
        }
    }