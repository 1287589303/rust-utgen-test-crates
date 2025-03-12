fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        for_both!(self, inner => inner.write_fmt(args))
    }