pub(crate) fn delimiter(&self) -> Delimiter {
        match self {
            Group::Compiler(g) => match g.delimiter() {
                proc_macro::Delimiter::Parenthesis => Delimiter::Parenthesis,
                proc_macro::Delimiter::Bracket => Delimiter::Bracket,
                proc_macro::Delimiter::Brace => Delimiter::Brace,
                proc_macro::Delimiter::None => Delimiter::None,
            },
            Group::Fallback(g) => g.delimiter(),
        }
    }