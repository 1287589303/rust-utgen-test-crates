fn read_line(&mut self, buf: &mut std::string::String) -> io::Result<usize> {
        for_both!(self, inner => inner.read_line(buf))
    }