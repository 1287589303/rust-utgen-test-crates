pub fn new(kind: MatchErrorKind) -> MatchError {
        #[cfg(feature = "alloc")]
        {
            MatchError(alloc::boxed::Box::new(kind))
        }
        #[cfg(not(feature = "alloc"))]
        {
            MatchError(kind)
        }
    }