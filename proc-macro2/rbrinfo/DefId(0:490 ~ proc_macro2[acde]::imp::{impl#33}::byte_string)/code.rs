pub(crate) fn byte_string(bytes: &[u8]) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler(proc_macro::Literal::byte_string(bytes))
        } else {
            Literal::Fallback(fallback::Literal::byte_string(bytes))
        }
    }