use core::mem::size_of_val;
use rand_core::block::{BlockRng, BlockRngCore, CryptoBlockRng};
use rand_core::{CryptoRng, RngCore, SeedableRng, TryCryptoRng, TryRngCore};
#[derive(Debug)]
struct ReseedingCore<R, Rsdr> {
    inner: R,
    reseeder: Rsdr,
    threshold: i64,
    bytes_until_reseed: i64,
}
impl<R, Rsdr> Clone for ReseedingCore<R, Rsdr>
where
    R: BlockRngCore + SeedableRng + Clone,
    Rsdr: TryRngCore + Clone,
{
    fn clone(&self) -> ReseedingCore<R, Rsdr> {
        ReseedingCore {
            inner: self.inner.clone(),
            reseeder: self.reseeder.clone(),
            threshold: self.threshold,
            bytes_until_reseed: 0,
        }
    }
}
