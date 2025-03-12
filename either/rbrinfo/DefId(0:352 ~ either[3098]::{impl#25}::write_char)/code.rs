fn write_char(&mut self, c: char) -> fmt::Result {
        for_both!(self, inner => inner.write_char(c))
    }