pub(crate) fn byte_character(byte: u8) -> Literal {
        if inside_proc_macro() {
            Literal::Compiler({
                #[cfg(not(no_literal_byte_character))]
                {
                    proc_macro::Literal::byte_character(byte)
                }

                #[cfg(no_literal_byte_character)]
                {
                    let fallback = fallback::Literal::byte_character(byte);
                    proc_macro::Literal::from_str_unchecked(&fallback.repr)
                }
            })
        } else {
            Literal::Fallback(fallback::Literal::byte_character(byte))
        }
    }