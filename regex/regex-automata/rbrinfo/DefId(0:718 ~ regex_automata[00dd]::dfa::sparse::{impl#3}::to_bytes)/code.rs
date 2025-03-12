fn to_bytes<E: Endian>(&self) -> Vec<u8> {
        let mut buf = vec![0; self.write_to_len()];
        // This should always succeed since the only possible serialization
        // error is providing a buffer that's too small, but we've ensured that
        // `buf` is big enough here.
        self.write_to::<E>(&mut buf).unwrap();
        buf
    }