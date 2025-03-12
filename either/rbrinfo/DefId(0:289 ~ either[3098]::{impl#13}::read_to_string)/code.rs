fn read_to_string(&mut self, buf: &mut std::string::String) -> io::Result<usize> {
        for_both!(self, inner => inner.read_to_string(buf))
    }