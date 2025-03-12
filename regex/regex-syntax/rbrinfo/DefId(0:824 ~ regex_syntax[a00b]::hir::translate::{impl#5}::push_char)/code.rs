fn push_char(&self, ch: char) {
        let mut buf = [0; 4];
        let bytes = ch.encode_utf8(&mut buf).as_bytes();
        let mut stack = self.trans().stack.borrow_mut();
        if let Some(HirFrame::Literal(ref mut literal)) = stack.last_mut() {
            literal.extend_from_slice(bytes);
        } else {
            stack.push(HirFrame::Literal(bytes.to_vec()));
        }
    }