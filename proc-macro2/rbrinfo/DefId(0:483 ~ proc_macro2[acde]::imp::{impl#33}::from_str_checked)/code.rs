pub(crate) fn from_str_checked(repr: &str) -> Result<Self, LexError> {
        if inside_proc_macro() {
            let literal = proc_macro::Literal::from_str_checked(repr)?;
            Ok(Literal::Compiler(literal))
        } else {
            let literal = fallback::Literal::from_str_checked(repr)?;
            Ok(Literal::Fallback(literal))
        }
    }