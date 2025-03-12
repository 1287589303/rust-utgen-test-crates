pub(crate) fn character(ch: char) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler(proc_macro::Literal::character(ch))
        } else {
            Literal::Fallback(fallback::Literal::character(ch))
        }
    }