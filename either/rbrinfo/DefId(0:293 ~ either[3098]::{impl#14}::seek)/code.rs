fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        for_both!(self, inner => inner.seek(pos))
    }