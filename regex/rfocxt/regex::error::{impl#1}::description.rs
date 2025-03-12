use alloc::string::{String, ToString};
use regex_automata::meta;
#[non_exhaustive]
#[derive(Clone, PartialEq)]
pub enum Error {
    /// A syntax error.
    Syntax(String),
    /// The compiled program exceeded the set size
    /// limit. The argument is the size limit imposed by
    /// [`RegexBuilder::size_limit`](crate::RegexBuilder::size_limit). Even
    /// when not configured explicitly, it defaults to a reasonable limit.
    ///
    /// If you're getting this error, it occurred because your regex has been
    /// compiled to an intermediate state that is too big. It is important to
    /// note that exceeding this limit does _not_ mean the regex is too big to
    /// _work_, but rather, the regex is big enough that it may wind up being
    /// surprisingly slow when used in a search. In other words, this error is
    /// meant to be a practical heuristic for avoiding a performance footgun,
    /// and especially so for the case where the regex pattern is coming from
    /// an untrusted source.
    ///
    /// There are generally two ways to move forward if you hit this error.
    /// The first is to find some way to use a smaller regex. The second is to
    /// increase the size limit via `RegexBuilder::size_limit`. However, if
    /// your regex pattern is not from a trusted source, then neither of these
    /// approaches may be appropriate. Instead, you'll have to determine just
    /// how big of a regex you want to allow.
    CompiledTooBig(usize),
}
#[cfg(feature = "std")]
impl std::error::Error for Error {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        match *self {
            Error::Syntax(ref err) => err,
            Error::CompiledTooBig(_) => "compiled program too big",
        }
    }
}
