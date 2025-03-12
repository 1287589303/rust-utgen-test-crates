fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group::Compiler(group) => Display::fmt(group, formatter),
            Group::Fallback(group) => Display::fmt(group, formatter),
        }
    }