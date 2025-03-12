use crate::{Equivalent, TryReserveError};
use core::hash::{BuildHasher, Hash};
use core::iter::{Chain, FusedIterator};
use core::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign,
};
use core::{fmt, mem};
use map::make_hash;
use super::map::{self, HashMap, Keys};
use crate::raw::{Allocator, Global, RawExtractIf};
use crate::DefaultHashBuilder;
pub struct Iter<'a, K> {
    iter: Keys<'a, K, ()>,
}
pub struct Keys<'a, K, V> {
    inner: Iter<'a, K, V>,
}
impl<K> Clone for Iter<'_, K> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Iter { iter: self.iter.clone() }
    }
}
