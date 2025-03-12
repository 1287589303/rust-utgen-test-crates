fn fill_buf(&mut self) -> io::Result<&[u8]> {
        for_both!(self, inner => inner.fill_buf())
    }