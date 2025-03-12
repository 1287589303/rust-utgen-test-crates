fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        for_both!(self, inner => inner.write_all(buf))
    }