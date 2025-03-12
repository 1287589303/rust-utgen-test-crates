pub fn map_either_with<Ctx, F, G, M, S>(self, ctx: Ctx, f: F, g: G) -> Either<M, S>
    where
        F: FnOnce(Ctx, L) -> M,
        G: FnOnce(Ctx, R) -> S,
    {
        match self {
            Left(l) => Left(f(ctx, l)),
            Right(r) => Right(g(ctx, r)),
        }
    }