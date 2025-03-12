fn set_len(&mut self, new_len: usize) {
        // The only way an accelerator gets added is if a state exists for
        // it, and if a state exists, then its index is guaranteed to be
        // representable by a AccelTy by virtue of the guarantees provided by
        // StateID.
        let new_len = AccelTy::try_from(new_len).unwrap();
        self.accels[0] = new_len;
    }