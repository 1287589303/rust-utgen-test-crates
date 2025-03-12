pub(crate) fn f64_unsuffixed(f: f64) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler(proc_macro::Literal::f64_unsuffixed(f))
        } else {
            Literal::Fallback(fallback::Literal::f64_unsuffixed(f))
        }
    }