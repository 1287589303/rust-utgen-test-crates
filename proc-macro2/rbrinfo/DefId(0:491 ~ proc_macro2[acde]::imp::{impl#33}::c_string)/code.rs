pub(crate) fn c_string(string: &CStr) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler({
                #[cfg(not(no_literal_c_string))]
                {
                    proc_macro::Literal::c_string(string)
                }

                #[cfg(no_literal_c_string)]
                {
                    let fallback = fallback::Literal::c_string(string);
                    proc_macro::Literal::from_str_unchecked(&fallback.repr)
                }
            })
        } else {
            Literal::Fallback(fallback::Literal::c_string(string))
        }
    }