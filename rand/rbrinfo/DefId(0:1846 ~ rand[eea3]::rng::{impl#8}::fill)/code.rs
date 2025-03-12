fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
                if self.len() > 0 {
                    rng.fill_bytes(self.as_mut_bytes());
                    for x in self {
                    *x = Wrapping(x.0.to_le());
                    }
                }
            }