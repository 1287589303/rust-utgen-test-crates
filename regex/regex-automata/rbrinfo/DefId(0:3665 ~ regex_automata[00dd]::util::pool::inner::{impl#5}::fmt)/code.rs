fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Pool")
                .field("stacks", &self.stacks)
                .field("owner", &self.owner)
                .field("owner_val", &self.owner_val)
                .finish()
        }