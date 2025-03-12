fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            InvalidBase64Details::UnexpectedSymbol(code_point) => {
                write!(f, "symbol with codepoint {} not expected", code_point)
            }
            InvalidBase64Details::AlphabetSymbolAfterPadding => {
                write!(f, "alphabet symbol present after padding")
            }
            InvalidBase64Details::LoneAlphabetSymbol => write!(f, "lone alphabet symbol present"),
            InvalidBase64Details::Padding => write!(f, "incorrect padding"),
        }
    }