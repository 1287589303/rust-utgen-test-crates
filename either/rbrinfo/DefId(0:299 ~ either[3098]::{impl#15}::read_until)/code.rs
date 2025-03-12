fn read_until(&mut self, byte: u8, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> {
        for_both!(self, inner => inner.read_until(byte, buf))
    }