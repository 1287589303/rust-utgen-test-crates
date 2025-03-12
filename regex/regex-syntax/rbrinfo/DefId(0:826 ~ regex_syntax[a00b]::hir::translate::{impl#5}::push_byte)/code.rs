fn push_byte(&self, byte: u8) {
        let mut stack = self.trans().stack.borrow_mut();
        if let Some(HirFrame::Literal(ref mut literal)) = stack.last_mut() {
            literal.push(byte);
        } else {
            stack.push(HirFrame::Literal(vec![byte]));
        }
    }