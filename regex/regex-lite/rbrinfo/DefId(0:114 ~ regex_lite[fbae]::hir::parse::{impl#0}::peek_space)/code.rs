fn peek_space(&self) -> Option<char> {
        if !self.flags().ignore_whitespace {
            return self.peek();
        }
        if self.is_done() {
            return None;
        }
        let mut start = self.pos() + self.char().len_utf8();
        let mut in_comment = false;
        for (i, ch) in self.pattern()[start..].char_indices() {
            if ch.is_whitespace() {
                continue;
            } else if !in_comment && ch == '#' {
                in_comment = true;
            } else if in_comment && ch == '\n' {
                in_comment = false;
            } else {
                start += i;
                break;
            }
        }
        self.pattern()[start..].chars().next()
    }