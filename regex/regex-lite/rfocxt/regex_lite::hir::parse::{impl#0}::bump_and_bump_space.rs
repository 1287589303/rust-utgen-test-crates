use core::cell::{Cell, RefCell};
use alloc::{
    boxed::Box, string::{String, ToString},
    vec, vec::Vec,
};
use crate::{error::Error, hir::{self, Config, Flags, Hir, HirKind}};
const ERR_TOO_MUCH_NESTING: &str = "pattern has too much nesting";
const ERR_TOO_MANY_CAPTURES: &str = "too many capture groups";
const ERR_DUPLICATE_CAPTURE_NAME: &str = "duplicate capture group name";
const ERR_UNCLOSED_GROUP: &str = "found open group without closing ')'";
const ERR_UNCLOSED_GROUP_QUESTION: &str = "expected closing ')', but got end of pattern";
const ERR_UNOPENED_GROUP: &str = "found closing ')' without matching '('";
const ERR_LOOK_UNSUPPORTED: &str = "look-around is not supported";
const ERR_EMPTY_FLAGS: &str = "empty flag directive '(?)' is not allowed";
const ERR_MISSING_GROUP_NAME: &str = "expected capture group name, but got end of pattern";
const ERR_INVALID_GROUP_NAME: &str = "invalid group name";
const ERR_UNCLOSED_GROUP_NAME: &str = "expected end of capture group name, but got end of pattern";
const ERR_EMPTY_GROUP_NAME: &str = "empty capture group names are not allowed";
const ERR_FLAG_UNRECOGNIZED: &str = "unrecognized inline flag";
const ERR_FLAG_REPEATED_NEGATION: &str = "inline flag negation cannot be repeated";
const ERR_FLAG_DUPLICATE: &str = "duplicate inline flag is not allowed";
const ERR_FLAG_UNEXPECTED_EOF: &str = "expected ':' or ')' to end inline flags, but got end of pattern";
const ERR_FLAG_DANGLING_NEGATION: &str = "inline flags cannot end with negation directive";
const ERR_DECIMAL_NO_DIGITS: &str = "expected decimal number, but found no digits";
const ERR_DECIMAL_INVALID: &str = "got invalid decimal number";
const ERR_HEX_BRACE_INVALID_DIGIT: &str = "expected hexadecimal number in braces, but got non-hex digit";
const ERR_HEX_BRACE_UNEXPECTED_EOF: &str = "expected hexadecimal number, but saw end of pattern before closing brace";
const ERR_HEX_BRACE_EMPTY: &str = "expected hexadecimal number in braces, but got no digits";
const ERR_HEX_BRACE_INVALID: &str = "got invalid hexadecimal number in braces";
const ERR_HEX_FIXED_UNEXPECTED_EOF: &str = "expected fixed length hexadecimal number, but saw end of pattern first";
const ERR_HEX_FIXED_INVALID_DIGIT: &str = "expected fixed length hexadecimal number, but got non-hex digit";
const ERR_HEX_FIXED_INVALID: &str = "got invalid fixed length hexadecimal number";
const ERR_HEX_UNEXPECTED_EOF: &str = "expected hexadecimal number, but saw end of pattern first";
const ERR_ESCAPE_UNEXPECTED_EOF: &str = "saw start of escape sequence, but saw end of pattern before it finished";
const ERR_BACKREF_UNSUPPORTED: &str = "backreferences are not supported";
const ERR_UNICODE_CLASS_UNSUPPORTED: &str = "Unicode character classes are not supported";
const ERR_ESCAPE_UNRECOGNIZED: &str = "unrecognized escape sequence";
const ERR_POSIX_CLASS_UNRECOGNIZED: &str = "unrecognized POSIX character class";
const ERR_UNCOUNTED_REP_SUB_MISSING: &str = "uncounted repetition operator must be applied to a sub-expression";
const ERR_COUNTED_REP_SUB_MISSING: &str = "counted repetition operator must be applied to a sub-expression";
const ERR_COUNTED_REP_UNCLOSED: &str = "found unclosed counted repetition operator";
const ERR_COUNTED_REP_MIN_UNCLOSED: &str = "found incomplete and unclosed counted repetition operator";
const ERR_COUNTED_REP_COMMA_UNCLOSED: &str = "found counted repetition operator with a comma that is unclosed";
const ERR_COUNTED_REP_MIN_MAX_UNCLOSED: &str = "found counted repetition with min and max that is unclosed";
const ERR_COUNTED_REP_INVALID: &str = "expected closing brace for counted repetition, but got something else";
const ERR_COUNTED_REP_INVALID_RANGE: &str = "found counted repetition with a min bigger than its max";
const ERR_CLASS_UNCLOSED_AFTER_ITEM: &str = "non-empty character class has no closing bracket";
const ERR_CLASS_INVALID_RANGE_ITEM: &str = "character class ranges must start and end with a single character";
const ERR_CLASS_INVALID_ITEM: &str = "invalid escape sequence in character class";
const ERR_CLASS_UNCLOSED_AFTER_DASH: &str = "non-empty character class has no closing bracket after dash";
const ERR_CLASS_UNCLOSED_AFTER_NEGATION: &str = "negated character class has no closing bracket";
const ERR_CLASS_UNCLOSED_AFTER_CLOSING: &str = "character class begins with literal ']' but has no closing bracket";
const ERR_CLASS_INVALID_RANGE: &str = "invalid range in character class";
const ERR_CLASS_UNCLOSED: &str = "found unclosed character class";
const ERR_CLASS_NEST_UNSUPPORTED: &str = "nested character classes are not supported";
const ERR_CLASS_INTERSECTION_UNSUPPORTED: &str = "character class intersection is not supported";
const ERR_CLASS_DIFFERENCE_UNSUPPORTED: &str = "character class difference is not supported";
const ERR_CLASS_SYMDIFFERENCE_UNSUPPORTED: &str = "character class symmetric difference is not supported";
const ERR_SPECIAL_WORD_BOUNDARY_UNCLOSED: &str = "special word boundary assertion is unclosed or has an invalid character";
const ERR_SPECIAL_WORD_BOUNDARY_UNRECOGNIZED: &str = "special word boundary assertion is unrecognized";
const ERR_SPECIAL_WORD_OR_REP_UNEXPECTED_EOF: &str = "found start of special word boundary or repetition without an end";
#[derive(Clone, Debug)]
pub(super) struct Parser<'a> {
    /// The configuration of the parser as given by the caller.
    config: Config,
    /// The pattern we're parsing as given by the caller.
    pattern: &'a str,
    /// The call depth of the parser. This is incremented for each
    /// sub-expression parsed. Its peak value is the maximum nesting of the
    /// pattern.
    depth: Cell<u32>,
    /// The current position of the parser.
    pos: Cell<usize>,
    /// The current codepoint of the parser. The codepoint corresponds to the
    /// codepoint encoded in `pattern` beginning at `pos`.
    ///
    /// This is `None` if and only if `pos == pattern.len()`.
    char: Cell<Option<char>>,
    /// The current capture index.
    capture_index: Cell<u32>,
    /// The flags that are currently set.
    flags: RefCell<Flags>,
    /// A sorted sequence of capture names. This is used to detect duplicate
    /// capture names and report an error if one is detected.
    capture_names: RefCell<Vec<String>>,
}
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
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub(crate) size_limit: Option<usize>,
}
impl<'a> Parser<'a> {
    pub(super) fn new(config: Config, pattern: &'a str) -> Parser<'a> {}
    fn pattern(&self) -> &str {}
    fn pos(&self) -> usize {}
    fn increment_depth(&self) -> Result<u32, Error> {}
    fn decrement_depth(&self) {}
    fn char(&self) -> char {
        self.char.get().expect("codepoint, but parser is done")
    }
    fn is_done(&self) -> bool {
        self.pos() == self.pattern.len()
    }
    fn flags(&self) -> Flags {}
    fn bump(&self) -> bool {
        if self.is_done() {
            return false;
        }
        self.pos.set(self.pos() + self.char().len_utf8());
        self.char.set(self.pattern()[self.pos()..].chars().next());
        self.char.get().is_some()
    }
    fn bump_if(&self, prefix: &str) -> bool {}
    fn bump_and_bump_space(&self) -> bool {
        if !self.bump() {
            return false;
        }
        self.bump_space();
        !self.is_done()
    }
    fn bump_space(&self) {
        if !self.flags().ignore_whitespace {
            return;
        }
        while !self.is_done() {
            if self.char().is_whitespace() {
                self.bump();
            } else if self.char() == '#' {
                self.bump();
                while !self.is_done() {
                    let c = self.char();
                    self.bump();
                    if c == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }
    fn peek(&self) -> Option<char> {}
    fn peek_space(&self) -> Option<char> {}
    fn next_capture_index(&self) -> Result<u32, Error> {}
    fn add_capture_name(&self, name: &str) -> Result<(), Error> {}
    fn is_lookaround_prefix(&self) -> bool {}
}
