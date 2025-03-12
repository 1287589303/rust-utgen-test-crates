fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error {
        ast::Error { kind, pattern: self.pattern().to_string(), span }
    }