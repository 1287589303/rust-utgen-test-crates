use crate::util::search::{Input, MatchError};
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
impl<'h> Input<'h> {
    #[inline]
    pub fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h> {}
    #[inline]
    pub fn span<S: Into<Span>>(mut self, span: S) -> Input<'h> {}
    #[inline]
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> Input<'h> {}
    #[inline]
    pub fn anchored(mut self, mode: Anchored) -> Input<'h> {}
    #[inline]
    pub fn earliest(mut self, yes: bool) -> Input<'h> {}
    #[inline]
    pub fn set_span<S: Into<Span>>(&mut self, span: S) {}
    #[inline]
    pub fn set_range<R: RangeBounds<usize>>(&mut self, range: R) {}
    #[inline]
    pub fn set_start(&mut self, start: usize) {
        self.set_span(Span { start, ..self.get_span() });
    }
    #[inline]
    pub fn set_end(&mut self, end: usize) {
        self.set_span(Span { end, ..self.get_span() });
    }
    #[inline]
    pub fn set_anchored(&mut self, mode: Anchored) {}
    #[inline]
    pub fn set_earliest(&mut self, yes: bool) {}
    #[inline]
    pub fn haystack(&self) -> &[u8] {}
    #[inline]
    pub fn start(&self) -> usize {
        self.get_span().start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.get_span().end
    }
    #[inline]
    pub fn get_span(&self) -> Span {}
    #[inline]
    pub fn get_range(&self) -> Range<usize> {}
    #[inline]
    pub fn get_anchored(&self) -> Anchored {
        self.anchored
    }
    #[inline]
    pub fn get_earliest(&self) -> bool {}
    #[inline]
    pub fn is_done(&self) -> bool {}
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {
        utf8::is_boundary(self.haystack(), offset)
    }
}
impl Anchored {
    #[inline]
    pub fn is_anchored(&self) -> bool {
        matches!(* self, Anchored::Yes | Anchored::Pattern(_))
    }
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {}
}
fn skip_splits<T, F>(
    forward: bool,
    input: &Input<'_>,
    init_value: T,
    mut match_offset: usize,
    mut find: F,
) -> Result<Option<T>, MatchError>
where
    F: FnMut(&Input<'_>) -> Result<Option<(T, usize)>, MatchError>,
{
    if input.get_anchored().is_anchored() {
        return Ok(
            if input.is_char_boundary(match_offset) { Some(init_value) } else { None },
        );
    }
    let mut value = init_value;
    let mut input = input.clone();
    while !input.is_char_boundary(match_offset) {
        if forward {
            input.set_start(input.start().checked_add(1).unwrap());
        } else {
            input
                .set_end(
                    match input.end().checked_sub(1) {
                        None => return Ok(None),
                        Some(end) => end,
                    },
                );
        }
        match find(&input)? {
            None => return Ok(None),
            Some((new_value, new_match_end)) => {
                value = new_value;
                match_offset = new_match_end;
            }
        }
    }
    Ok(Some(value))
}
