fn to_bytes<E: Endian>(&self) -> (Vec<u8>, usize) {
        let len = self.write_to_len();
        let (mut buf, padding) = wire::alloc_aligned_buffer::<u32>(len);
        // This should always succeed since the only possible serialization
        // error is providing a buffer that's too small, but we've ensured that
        // `buf` is big enough here.
        self.as_ref().write_to::<E>(&mut buf[padding..]).unwrap();
        (buf, padding)
    }