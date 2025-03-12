pub(crate) unsafe fn from_str_unchecked(repr: &str) -> Self {
        if inside_proc_macro() {
            Literal::Compiler(proc_macro::Literal::from_str_unchecked(repr))
        } else {
            Literal::Fallback(unsafe { fallback::Literal::from_str_unchecked(repr) })
        }
    }