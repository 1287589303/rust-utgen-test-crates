use alloc::{boxed::Box, string::String, vec, vec::Vec};
use crate::{error::Error, utf8};
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    /// The maximum number of times we're allowed to recurse.
    ///
    /// Note that unlike the regex-syntax parser, we actually use recursion in
    /// this parser for simplicity. My hope is that by setting a conservative
    /// default call limit and providing a way to configure it, that we can
    /// keep this simplification. But if we must, we can re-work the parser to
    /// put the call stack on the heap like regex-syntax does.
    pub(crate) nest_limit: u32,
    /// Various flags that control how a pattern is interpreted.
    pub(crate) flags: Flags,
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Flags {
    /// Whether to match case insensitively.
    ///
    /// This is the `i` flag.
    pub(crate) case_insensitive: bool,
    /// Whether `^` and `$` should be treated as line anchors or not.
    ///
    /// This is the `m` flag.
    pub(crate) multi_line: bool,
    /// Whether `.` should match line terminators or not.
    ///
    /// This is the `s` flag.
    pub(crate) dot_matches_new_line: bool,
    /// Whether to swap the meaning of greedy and non-greedy operators.
    ///
    /// This is the `U` flag.
    pub(crate) swap_greed: bool,
    /// Whether to enable CRLF mode.
    ///
    /// This is the `R` flag.
    pub(crate) crlf: bool,
    /// Whether to ignore whitespace. i.e., verbose mode.
    ///
    /// This is the `x` flag.
    pub(crate) ignore_whitespace: bool,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            nest_limit: 50,
            flags: Flags::default(),
        }
    }
}
