fn next_utf8(&mut self) -> Option<(char, &'i str)> {
        loop {
            let utf8 = self.chars.as_str();
            match self.chars.next() {
                Some(c) => {
                    if !ascii_tab_or_new_line(c) {
                        return Some((c, &utf8[..c.len_utf8()]));
                    }
                }
                None => return None,
            }
        }
    }