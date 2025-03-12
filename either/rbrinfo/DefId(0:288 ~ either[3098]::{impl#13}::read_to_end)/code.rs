fn read_to_end(&mut self, buf: &mut std::vec::Vec<u8>) -> io::Result<usize> {
        for_both!(self, inner => inner.read_to_end(buf))
    }