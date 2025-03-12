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
