fn new(p: &'p ParserI<'s, P>) -> NestLimiter<'p, 's, P> {
        NestLimiter { p, depth: 0 }
    }