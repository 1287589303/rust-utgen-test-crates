fn write_fmt(&mut self, fmt: fmt::Arguments<'_>) -> io::Result<()> {
        for_both!(self, inner => inner.write_fmt(fmt))
    }