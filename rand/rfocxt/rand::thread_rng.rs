pub use rand_core;
pub use rand_core::{CryptoRng, RngCore, SeedableRng, TryCryptoRng, TryRngCore};
#[cfg(feature = "thread_rng")]
pub use crate::rngs::thread::rng;
pub use rng::{Fill, Rng};
#[cfg(feature = "thread_rng")]
use crate::distr::{Distribution, StandardUniform};
#[derive(Clone)]
pub struct ThreadRng {
    rng: Rc<UnsafeCell<ReseedingRng<Core, OsRng>>>,
}
#[cfg(feature = "thread_rng")]
#[deprecated(since = "0.9.0", note = "Renamed to `rng`")]
#[inline]
pub fn thread_rng() -> crate::rngs::ThreadRng {
    rng()
}
pub fn rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}
