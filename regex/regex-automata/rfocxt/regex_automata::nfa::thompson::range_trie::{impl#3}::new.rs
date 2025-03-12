use core::{cell::RefCell, fmt, mem, ops::RangeInclusive};
use alloc::{format, string::String, vec, vec::Vec};
use regex_syntax::utf8::Utf8Range;
use crate::util::primitives::StateID;
const FINAL: StateID = StateID::ZERO;
const ROOT: StateID = StateID::new_unchecked(1);
#[derive(Clone, Debug, Eq, PartialEq)]
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SplitRange {
    Old(Utf8Range),
    New(Utf8Range),
    Both(Utf8Range),
}
impl Split {
    fn new(o: Utf8Range, n: Utf8Range) -> Option<Split> {
        let range = |r: RangeInclusive<u8>| Utf8Range {
            start: *r.start(),
            end: *r.end(),
        };
        let old = |r| SplitRange::Old(range(r));
        let new = |r| SplitRange::New(range(r));
        let both = |r| SplitRange::Both(range(r));
        let (a, b, x, y) = (o.start, o.end, n.start, n.end);
        if b < x || y < a {
            None
        } else if a == x && b == y {
            Some(Split::parts1(both(a..=b)))
        } else if a == x && b < y {
            Some(Split::parts2(both(a..=b), new(b + 1..=y)))
        } else if b == y && a > x {
            Some(Split::parts2(new(x..=a - 1), both(a..=b)))
        } else if x == a && y < b {
            Some(Split::parts2(both(x..=y), old(y + 1..=b)))
        } else if y == b && x > a {
            Some(Split::parts2(old(a..=x - 1), both(x..=y)))
        } else if a > x && b < y {
            Some(Split::parts3(new(x..=a - 1), both(a..=b), new(b + 1..=y)))
        } else if x > a && y < b {
            Some(Split::parts3(old(a..=x - 1), both(x..=y), old(y + 1..=b)))
        } else if b == x && a < y {
            Some(Split::parts3(old(a..=b - 1), both(b..=b), new(b + 1..=y)))
        } else if y == a && x < b {
            Some(Split::parts3(new(x..=y - 1), both(y..=y), old(y + 1..=b)))
        } else if b > x && b < y {
            Some(Split::parts3(old(a..=x - 1), both(x..=b), new(b + 1..=y)))
        } else if y > a && y < b {
            Some(Split::parts3(new(x..=a - 1), both(a..=y), old(y + 1..=b)))
        } else {
            unreachable!()
        }
    }
    fn parts1(r1: SplitRange) -> Split {
        let nada = SplitRange::Old(Utf8Range { start: 0, end: 0 });
        Split {
            partitions: [r1, nada, nada],
            len: 1,
        }
    }
    fn parts2(r1: SplitRange, r2: SplitRange) -> Split {
        let nada = SplitRange::Old(Utf8Range { start: 0, end: 0 });
        Split {
            partitions: [r1, r2, nada],
            len: 2,
        }
    }
    fn parts3(r1: SplitRange, r2: SplitRange, r3: SplitRange) -> Split {
        Split {
            partitions: [r1, r2, r3],
            len: 3,
        }
    }
    fn as_slice(&self) -> &[SplitRange] {}
}
