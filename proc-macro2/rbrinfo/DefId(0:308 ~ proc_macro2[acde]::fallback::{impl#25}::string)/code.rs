pub(crate) fn string(string: &str) -> Literal {
        let mut repr = String::with_capacity(string.len() + 2);
        repr.push('"');
        escape_utf8(string, &mut repr);
        repr.push('"');
        Literal::_new(repr)
    }