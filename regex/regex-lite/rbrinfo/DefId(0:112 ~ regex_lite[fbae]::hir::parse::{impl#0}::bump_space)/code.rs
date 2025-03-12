fn bump_space(&self) {
        if !self.flags().ignore_whitespace {
            return;
        }
        while !self.is_done() {
            if self.char().is_whitespace() {
                self.bump();
            } else if self.char() == '#' {
                self.bump();
                while !self.is_done() {
                    let c = self.char();
                    self.bump();
                    if c == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }