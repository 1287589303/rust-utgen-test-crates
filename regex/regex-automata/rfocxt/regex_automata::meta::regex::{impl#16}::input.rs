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
pub struct SplitN<'r, 'h> {
    splits: Split<'r, 'h>,
    limit: usize,
}
#[derive(Debug)]
pub struct Split<'r, 'h> {
    finder: FindMatches<'r, 'h>,
    last: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
impl<'r, 'h> SplitN<'r, 'h> {
    #[inline]
    pub fn input<'s>(&'s self) -> &'s Input<'h> {
        self.splits.input()
    }
}
impl<'r, 'h> Split<'r, 'h> {
    #[inline]
    pub fn input<'s>(&'s self) -> &'s Input<'h> {
        self.finder.input()
    }
}
