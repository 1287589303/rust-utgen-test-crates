unsafe fn next_state_unchecked(
        &self,
        current: StateID,
        byte: u8,
    ) -> StateID {
        // We don't (or shouldn't) need an unchecked variant for the byte
        // class mapping, since bound checks should be omitted automatically
        // by virtue of its representation. If this ends up not being true as
        // confirmed by codegen, please file an issue. ---AG
        let class = self.byte_classes().get(byte);
        let o = current.as_usize() + usize::from(class);
        let next = *self.trans().get_unchecked(o);
        next
    }