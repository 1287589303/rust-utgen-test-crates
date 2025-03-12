fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rs = self
            .transitions
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", rs)
    }