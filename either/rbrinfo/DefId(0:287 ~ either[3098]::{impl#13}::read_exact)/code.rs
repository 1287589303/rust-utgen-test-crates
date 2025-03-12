fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        for_both!(self, inner => inner.read_exact(buf))
    }