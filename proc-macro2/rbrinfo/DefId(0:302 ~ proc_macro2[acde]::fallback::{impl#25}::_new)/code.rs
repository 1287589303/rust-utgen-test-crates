pub(crate) fn _new(repr: String) -> Self {
        Literal {
            repr,
            span: Span::call_site(),
        }
    }