pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    #[inline]
    pub fn is_match(&self, haystack: &str) -> bool {}
    #[inline]
    pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {}
    #[inline]
    pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h> {}
    #[inline]
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_iter<'r, 'h>(&'r self, haystack: &'h str) -> CaptureMatches<'r, 'h> {}
    #[inline]
    pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h> {}
    #[inline]
    pub fn splitn<'r, 'h>(&'r self, haystack: &'h str, limit: usize) -> SplitN<'r, 'h> {}
    #[inline]
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {}
    #[inline]
    pub fn replace_all<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        rep: R,
    ) -> Cow<'h, str> {
        self.replacen(haystack, 0, rep)
    }
    #[inline]
    pub fn replacen<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'h, str> {
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(haystack).enumerate().peekable();
            if it.peek().is_none() {
                return Cow::Borrowed(haystack);
            }
            let mut new = String::with_capacity(haystack.len());
            let mut last_match = 0;
            for (i, m) in it {
                new.push_str(&haystack[last_match..m.start()]);
                new.push_str(&rep);
                last_match = m.end();
                if limit > 0 && i >= limit - 1 {
                    break;
                }
            }
            new.push_str(&haystack[last_match..]);
            return Cow::Owned(new);
        }
        let mut it = self.captures_iter(haystack).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(haystack);
        }
        let mut new = String::with_capacity(haystack.len());
        let mut last_match = 0;
        for (i, cap) in it {
            let m = cap.get(0).unwrap();
            new.push_str(&haystack[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
            if limit > 0 && i >= limit - 1 {
                break;
            }
        }
        new.push_str(&haystack[last_match..]);
        Cow::Owned(new)
    }
}
