pub(crate) fn from_meta_build_error(err: meta::BuildError) -> Error {
        if let Some(size_limit) = err.size_limit() {
            Error::CompiledTooBig(size_limit)
        } else if let Some(ref err) = err.syntax_error() {
            Error::Syntax(err.to_string())
        } else {
            // This is a little suspect. Technically there are more ways for
            // a meta regex to fail to build other than "exceeded size limit"
            // and "syntax error." For example, if there are too many states
            // or even too many patterns. But in practice this is probably
            // good enough. The worst thing that happens is that Error::Syntax
            // represents an error that isn't technically a syntax error, but
            // the actual message will still be shown. So... it's not too bad.
            //
            // We really should have made the Error type in the regex crate
            // completely opaque. Rookie mistake.
            Error::Syntax(err.to_string())
        }
    }