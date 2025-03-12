pub(crate) fn string(string: &str) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler(proc_macro::Literal::string(string))
        } else {
            Literal::Fallback(fallback::Literal::string(string))
        }
    }