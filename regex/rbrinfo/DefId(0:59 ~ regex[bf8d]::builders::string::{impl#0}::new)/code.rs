pub fn new(pattern: &str) -> RegexBuilder {
            RegexBuilder { builder: Builder::new([pattern]) }
        }