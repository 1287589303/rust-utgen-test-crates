fn flush(&mut self) -> io::Result<()> {
        for_both!(self, inner => inner.flush())
    }