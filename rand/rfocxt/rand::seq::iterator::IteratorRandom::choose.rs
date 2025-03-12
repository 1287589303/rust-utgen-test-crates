use super::coin_flipper::CoinFlipper;
#[allow(unused)]
use super::IndexedRandom;
use crate::Rng;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
pub trait IteratorRandom: Iterator + Sized {
    fn choose<R>(mut self, rng: &mut R) -> Option<Self::Item>
    where
        R: Rng + ?Sized,
    {
        let (mut lower, mut upper) = self.size_hint();
        let mut result = None;
        if upper == Some(lower) {
            return match lower {
                0 => None,
                1 => self.next(),
                _ => self.nth(rng.random_range(..lower)),
            };
        }
        let mut coin_flipper = CoinFlipper::new(rng);
        let mut consumed = 0;
        loop {
            if lower > 1 {
                let ix = coin_flipper.rng.random_range(..lower + consumed);
                let skip = if ix < lower {
                    result = self.nth(ix);
                    lower - (ix + 1)
                } else {
                    lower
                };
                if upper == Some(lower) {
                    return result;
                }
                consumed += lower;
                if skip > 0 {
                    self.nth(skip - 1);
                }
            } else {
                let elem = self.next();
                if elem.is_none() {
                    return result;
                }
                consumed += 1;
                if coin_flipper.random_ratio_one_over(consumed) {
                    result = elem;
                }
            }
            let hint = self.size_hint();
            lower = hint.0;
            upper = hint.1;
        }
    }
    fn choose_stable<R>(mut self, rng: &mut R) -> Option<Self::Item>
    where
        R: Rng + ?Sized,
    {
        let mut consumed = 0;
        let mut result = None;
        let mut coin_flipper = CoinFlipper::new(rng);
        loop {
            let mut next = 0;
            let (lower, _) = self.size_hint();
            if lower >= 2 {
                let highest_selected = (0..lower)
                    .filter(|ix| coin_flipper.random_ratio_one_over(consumed + ix + 1))
                    .last();
                consumed += lower;
                next = lower;
                if let Some(ix) = highest_selected {
                    result = self.nth(ix);
                    next -= ix + 1;
                    debug_assert!(
                        result.is_some(), "iterator shorter than size_hint().0"
                    );
                }
            }
            let elem = self.nth(next);
            if elem.is_none() {
                return result;
            }
            if coin_flipper.random_ratio_one_over(consumed + 1) {
                result = elem;
            }
            consumed += 1;
        }
    }
    fn choose_multiple_fill<R>(mut self, rng: &mut R, buf: &mut [Self::Item]) -> usize
    where
        R: Rng + ?Sized;
    #[cfg(feature = "alloc")]
    fn choose_multiple<R>(mut self, rng: &mut R, amount: usize) -> Vec<Self::Item>
    where
        R: Rng + ?Sized,
    {
        let mut reservoir = Vec::with_capacity(amount);
        reservoir.extend(self.by_ref().take(amount));
        if reservoir.len() == amount {
            for (i, elem) in self.enumerate() {
                let k = rng.random_range(..i + 1 + amount);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        } else {
            reservoir.shrink_to_fit();
        }
        reservoir
    }
}
pub(crate) struct CoinFlipper<R: RngCore> {
    pub rng: R,
    chunk: u32,
    chunk_remaining: u32,
}
impl<R: RngCore> CoinFlipper<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            chunk: 0,
            chunk_remaining: 0,
        }
    }
    #[inline]
    pub fn random_ratio_one_over(&mut self, d: usize) -> bool {
        debug_assert_ne!(d, 0);
        let c = (usize::BITS - 1 - d.leading_zeros()).min(32);
        if self.flip_c_heads(c) {
            let numerator = 1 << c;
            self.random_ratio(numerator, d)
        } else {
            false
        }
    }
    #[inline]
    fn random_ratio(&mut self, mut n: usize, d: usize) -> bool {}
    fn flip_c_heads(&mut self, mut c: u32) -> bool {}
}
