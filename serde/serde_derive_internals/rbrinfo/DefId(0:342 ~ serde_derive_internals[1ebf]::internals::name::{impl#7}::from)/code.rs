fn from(lit: &LitStr) -> Self {
        Name {
            value: lit.value(),
            span: lit.span(),
        }
    }