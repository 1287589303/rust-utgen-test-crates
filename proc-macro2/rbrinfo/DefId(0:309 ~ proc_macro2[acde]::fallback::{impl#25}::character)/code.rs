pub(crate) fn character(ch: char) -> Literal {
        let mut repr = String::new();
        repr.push('\'');
        if ch == '"' {
            // escape_debug turns this into '\"' which is unnecessary.
            repr.push(ch);
        } else {
            repr.extend(ch.escape_debug());
        }
        repr.push('\'');
        Literal::_new(repr)
    }