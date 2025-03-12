fn error(&self, span: Span, kind: ErrorKind) -> Error {
        Error { kind, pattern: self.pattern.to_string(), span }
    }