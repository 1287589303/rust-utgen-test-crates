fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group::Compiler(group) => Debug::fmt(group, formatter),
            Group::Fallback(group) => Debug::fmt(group, formatter),
        }
    }