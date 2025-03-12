type CachePool = Pool<Cache, CachePoolFn>;
type CachePoolGuard<'a> = PoolGuard<'a, Cache, CachePoolFn>;
type CachePoolFn = Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>;
use core::{borrow::Borrow, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use regex_syntax::{ast, hir::{self, Hir}};
use crate::{
    meta::{
        error::BuildError, strategy::{self, Strategy},
        wrappers,
    },
    nfa::thompson::WhichCaptures,
    util::{
        captures::{Captures, GroupInfo},
        iter, pool::{Pool, PoolGuard},
        prefilter::Prefilter, primitives::{NonMaxUsize, PatternID},
        search::{HalfMatch, Input, Match, MatchKind, PatternSet, Span},
    },
};
#[derive(Debug)]
pub struct Split<'r, 'h> {
    finder: FindMatches<'r, 'h>,
    last: usize,
}
#[derive(Debug)]
pub struct FindMatches<'r, 'h> {
    re: &'r Regex,
    cache: CachePoolGuard<'r>,
    it: iter::Searcher<'h>,
}
#[derive(Debug)]
pub struct FindMatches<'r, 'h, A> {
    re: &'r Regex<A>,
    it: iter::Searcher<'h>,
}
#[derive(Debug)]
pub struct FindMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: Captures,
    it: iter::Searcher<'h>,
}
#[derive(Debug)]
pub struct FindMatches<'r, 'c, 'h> {
    re: &'r Regex,
    cache: &'c mut Cache,
    it: iter::Searcher<'h>,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
impl<'r, 'h> Split<'r, 'h> {
    #[inline]
    pub fn input<'s>(&'s self) -> &'s Input<'h> {
        self.finder.input()
    }
}
impl<'r, 'h> FindMatches<'r, 'h> {
    #[inline]
    pub fn regex(&self) -> &'r Regex {}
    #[inline]
    pub fn input<'s>(&'s self) -> &'s Input<'h> {
        self.it.input()
    }
}
