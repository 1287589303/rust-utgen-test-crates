fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Url::parse(s)
    }