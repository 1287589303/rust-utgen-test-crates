fn from(ident: &Ident) -> Self {
        Name {
            value: ident.to_string(),
            span: ident.span(),
        }
    }