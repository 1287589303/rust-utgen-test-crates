pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h [u8],
    start: usize,
    end: usize,
}
pub struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
#[derive(Debug)]
pub struct CaptureMatches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::CapturesMatches<'r, 'h>,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Debug)]
pub struct Matches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::FindMatches<'r, 'h>,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    #[inline]
    pub fn is_match(&self, haystack: &[u8]) -> bool {}
    #[inline]
    pub fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>> {}
    #[inline]
    pub fn find_iter<'r, 'h>(&'r self, haystack: &'h [u8]) -> Matches<'r, 'h> {
        Matches {
            haystack,
            it: self.meta.find_iter(haystack),
        }
    }
    #[inline]
    pub fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_iter<'r, 'h>(
        &'r self,
        haystack: &'h [u8],
    ) -> CaptureMatches<'r, 'h> {
        CaptureMatches {
            haystack,
            it: self.meta.captures_iter(haystack),
        }
    }
    #[inline]
    pub fn split<'r, 'h>(&'r self, haystack: &'h [u8]) -> Split<'r, 'h> {}
    #[inline]
    pub fn splitn<'r, 'h>(&'r self, haystack: &'h [u8], limit: usize) -> SplitN<'r, 'h> {}
    #[inline]
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]> {}
    #[inline]
    pub fn replace_all<'h, R: Replacer>(
        &self,
        haystack: &'h [u8],
        rep: R,
    ) -> Cow<'h, [u8]> {}
    #[inline]
    pub fn replacen<'h, R: Replacer>(
        &self,
        haystack: &'h [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'h, [u8]> {
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(haystack).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(haystack);
            }
            let mut new = Vec::with_capacity(haystack.len());
            let mut last_match = 0;
            for (i, m) in it {
                new.extend_from_slice(&haystack[last_match..m.start()]);
                new.extend_from_slice(&rep);
                last_match = m.end();
                if limit > 0 && i >= limit - 1 {
                    break;
                }
            }
            new.extend_from_slice(&haystack[last_match..]);
            return Cow::Owned(new);
        }
        let mut it = self.captures_iter(haystack).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(haystack);
        }
        let mut new = Vec::with_capacity(haystack.len());
        let mut last_match = 0;
        for (i, cap) in it {
            let m = cap.get(0).unwrap();
            new.extend_from_slice(&haystack[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
            if limit > 0 && i >= limit - 1 {
                break;
            }
        }
        new.extend_from_slice(&haystack[last_match..]);
        Cow::Owned(new)
    }
}
impl<'h> Match<'h> {
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn as_bytes(&self) -> &'h [u8] {}
    #[inline]
    fn new(haystack: &'h [u8], start: usize, end: usize) -> Match<'h> {}
}
impl<'h> Captures<'h> {
    #[inline]
    pub fn get(&self, i: usize) -> Option<Match<'h>> {
        self.caps.get_group(i).map(|sp| Match::new(self.haystack, sp.start, sp.end))
    }
    #[inline]
    pub fn name(&self, name: &str) -> Option<Match<'h>> {}
    pub fn extract<const N: usize>(&self) -> (&'h [u8], [&'h [u8]; N]) {}
    #[inline]
    pub fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>) {}
    #[inline]
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {}
    #[inline]
    pub fn len(&self) -> usize {}
}
