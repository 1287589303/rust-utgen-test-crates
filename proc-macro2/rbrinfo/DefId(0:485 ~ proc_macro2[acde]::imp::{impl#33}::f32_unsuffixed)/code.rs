pub(crate) fn f32_unsuffixed(f: f32) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler(proc_macro::Literal::f32_unsuffixed(f))
        } else {
            Literal::Fallback(fallback::Literal::f32_unsuffixed(f))
        }
    }