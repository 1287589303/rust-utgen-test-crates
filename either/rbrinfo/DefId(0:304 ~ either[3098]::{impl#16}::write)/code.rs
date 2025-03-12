fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for_both!(self, inner => inner.write(buf))
    }