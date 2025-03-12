fn escape_utf8(string: &str, repr: &mut String) {
    let mut chars = string.chars();
    while let Some(ch) = chars.next() {
        if ch == '\0' {
            repr.push_str(
                if chars
                    .as_str()
                    .starts_with(|next| '0' <= next && next <= '7')
                {
                    // circumvent clippy::octal_escapes lint
                    r"\x00"
                } else {
                    r"\0"
                },
            );
        } else if ch == '\'' {
            // escape_debug turns this into "\'" which is unnecessary.
            repr.push(ch);
        } else {
            repr.extend(ch.escape_debug());
        }
    }
}