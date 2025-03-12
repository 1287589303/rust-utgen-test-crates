use core::{borrow::Borrow, fmt::Debug, panic::{RefUnwindSafe, UnwindSafe}};
#[cfg(feature = "alloc")]
use alloc::sync::Arc;
#[cfg(feature = "syntax")]
use regex_syntax::hir::{literal, Hir};
use crate::util::search::{MatchKind, Span};
pub(crate) use crate::util::prefilter::{
    aho_corasick::AhoCorasick, byteset::ByteSet, memchr::{Memchr, Memchr2, Memchr3},
    memmem::Memmem, teddy::Teddy,
};
#[derive(Clone, Debug)]
pub(crate) struct Memchr(u8);
#[derive(Clone, Debug)]
pub(crate) struct Memchr3(u8, u8, u8);
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone, Debug)]
pub(crate) struct Memmem {
    #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))]
    _unused: (),
    #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
    finder: memchr::memmem::Finder<'static>,
}
#[derive(Clone, Debug)]
pub(crate) struct Teddy {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    /// The actual Teddy searcher.
    ///
    /// Technically, it's possible that Teddy doesn't actually get used, since
    /// Teddy does require its haystack to at least be of a certain size
    /// (usually around the size of whatever vector is being used, so ~16
    /// or ~32 bytes). For haystacks shorter than that, the implementation
    /// currently uses Rabin-Karp.
    #[cfg(feature = "perf-literal-multisubstring")]
    searcher: aho_corasick::packed::Searcher,
    /// When running an anchored search, the packed searcher can't handle it so
    /// we defer to Aho-Corasick itself. Kind of sad, but changing the packed
    /// searchers to support anchored search would be difficult at worst and
    /// annoying at best. Since packed searchers only apply to small numbers of
    /// literals, we content ourselves that this is not much of an added cost.
    /// (That packed searchers only work with a small number of literals is
    /// also why we use a DFA here. Otherwise, the memory usage of a DFA would
    /// likely be unacceptable.)
    #[cfg(feature = "perf-literal-multisubstring")]
    anchored_ac: aho_corasick::dfa::DFA,
    /// The length of the smallest literal we look for.
    ///
    /// We use this as a heuristic to figure out whether this will be "fast" or
    /// not. Generally, the longer the better, because longer needles are more
    /// discriminating and thus reduce false positive rate.
    #[cfg(feature = "perf-literal-multisubstring")]
    minimum_len: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct AhoCorasick {
    #[cfg(not(feature = "perf-literal-multisubstring"))]
    _unused: (),
    #[cfg(feature = "perf-literal-multisubstring")]
    ac: aho_corasick::AhoCorasick,
}
#[derive(Clone, Debug)]
pub(crate) struct Memchr2(u8, u8);
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Debug)]
pub(crate) enum Choice {
    Memchr(Memchr),
    Memchr2(Memchr2),
    Memchr3(Memchr3),
    Memmem(Memmem),
    Teddy(Teddy),
    ByteSet(ByteSet),
    AhoCorasick(AhoCorasick),
}
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
impl Choice {
    pub(crate) fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Choice> {
        if needles.len() == 0 {
            debug!("prefilter building failed: found empty set of literals");
            return None;
        }
        if needles.iter().any(|n| n.as_ref().is_empty()) {
            debug!("prefilter building failed: literals match empty string");
            return None;
        }
        if let Some(pre) = Memchr::new(kind, needles) {
            debug!("prefilter built: memchr");
            return Some(Choice::Memchr(pre));
        }
        if let Some(pre) = Memchr2::new(kind, needles) {
            debug!("prefilter built: memchr2");
            return Some(Choice::Memchr2(pre));
        }
        if let Some(pre) = Memchr3::new(kind, needles) {
            debug!("prefilter built: memchr3");
            return Some(Choice::Memchr3(pre));
        }
        if let Some(pre) = Memmem::new(kind, needles) {
            debug!("prefilter built: memmem");
            return Some(Choice::Memmem(pre));
        }
        if let Some(pre) = Teddy::new(kind, needles) {
            debug!("prefilter built: teddy");
            return Some(Choice::Teddy(pre));
        }
        if let Some(pre) = ByteSet::new(kind, needles) {
            debug!("prefilter built: byteset");
            return Some(Choice::ByteSet(pre));
        }
        if let Some(pre) = AhoCorasick::new(kind, needles) {
            debug!("prefilter built: aho-corasick");
            return Some(Choice::AhoCorasick(pre));
        }
        debug!("prefilter building failed: no strategy could be found");
        None
    }
}
impl Memchr {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memchr> {
        #[cfg(not(feature = "perf-literal-substring"))] { None }
        #[cfg(feature = "perf-literal-substring")]
        {
            if needles.len() != 1 {
                return None;
            }
            if needles[0].as_ref().len() != 1 {
                return None;
            }
            Some(Memchr(needles[0].as_ref()[0]))
        }
    }
}
impl Memchr3 {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memchr3> {
        #[cfg(not(feature = "perf-literal-substring"))] { None }
        #[cfg(feature = "perf-literal-substring")]
        {
            if needles.len() != 3 {
                return None;
            }
            if !needles.iter().all(|n| n.as_ref().len() == 1) {
                return None;
            }
            let b1 = needles[0].as_ref()[0];
            let b2 = needles[1].as_ref()[0];
            let b3 = needles[2].as_ref()[0];
            Some(Memchr3(b1, b2, b3))
        }
    }
}
impl ByteSet {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<ByteSet> {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { None }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let mut set = [false; 256];
            for needle in needles.iter() {
                let needle = needle.as_ref();
                if needle.len() != 1 {
                    return None;
                }
                set[usize::from(needle[0])] = true;
            }
            Some(ByteSet(set))
        }
    }
}
impl Memmem {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memmem> {
        #[cfg(not(all(feature = "std", feature = "perf-literal-substring")))] { None }
        #[cfg(all(feature = "std", feature = "perf-literal-substring"))]
        {
            if needles.len() != 1 {
                return None;
            }
            let needle = needles[0].as_ref();
            let finder = memchr::memmem::Finder::new(needle).into_owned();
            Some(Memmem { finder })
        }
    }
}
impl Teddy {
    pub(crate) fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Teddy> {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { None }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let (packed_match_kind, ac_match_kind) = match kind {
                MatchKind::LeftmostFirst | MatchKind::All => {
                    (
                        aho_corasick::packed::MatchKind::LeftmostFirst,
                        aho_corasick::MatchKind::LeftmostFirst,
                    )
                }
            };
            let minimum_len = needles
                .iter()
                .map(|n| n.as_ref().len())
                .min()
                .unwrap_or(0);
            let packed = aho_corasick::packed::Config::new()
                .match_kind(packed_match_kind)
                .builder()
                .extend(needles)
                .build()?;
            let anchored_ac = aho_corasick::dfa::DFA::builder()
                .match_kind(ac_match_kind)
                .start_kind(aho_corasick::StartKind::Anchored)
                .prefilter(false)
                .build(needles)
                .ok()?;
            Some(Teddy {
                searcher: packed,
                anchored_ac,
                minimum_len,
            })
        }
    }
}
impl AhoCorasick {
    pub(crate) fn new<B: AsRef<[u8]>>(
        kind: MatchKind,
        needles: &[B],
    ) -> Option<AhoCorasick> {
        #[cfg(not(feature = "perf-literal-multisubstring"))] { None }
        #[cfg(feature = "perf-literal-multisubstring")]
        {
            let ac_match_kind = match kind {
                MatchKind::LeftmostFirst | MatchKind::All => {
                    aho_corasick::MatchKind::LeftmostFirst
                }
            };
            let ac_kind = if needles.len() <= 500 {
                aho_corasick::AhoCorasickKind::DFA
            } else {
                aho_corasick::AhoCorasickKind::ContiguousNFA
            };
            let result = aho_corasick::AhoCorasick::builder()
                .kind(Some(ac_kind))
                .match_kind(ac_match_kind)
                .start_kind(aho_corasick::StartKind::Both)
                .prefilter(false)
                .build(needles);
            let ac = match result {
                Ok(ac) => ac,
                Err(_err) => {
                    debug!("aho-corasick prefilter failed to build: {}", _err);
                    return None;
                }
            };
            Some(AhoCorasick { ac })
        }
    }
}
impl Memchr2 {
    pub(crate) fn new<B: AsRef<[u8]>>(
        _kind: MatchKind,
        needles: &[B],
    ) -> Option<Memchr2> {
        #[cfg(not(feature = "perf-literal-substring"))] { None }
        #[cfg(feature = "perf-literal-substring")]
        {
            if needles.len() != 2 {
                return None;
            }
            if !needles.iter().all(|n| n.as_ref().len() == 1) {
                return None;
            }
            let b1 = needles[0].as_ref()[0];
            let b2 = needles[1].as_ref()[0];
            Some(Memchr2(b1, b2))
        }
    }
}
