fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for_both!(self, inner => inner.read(buf))
    }