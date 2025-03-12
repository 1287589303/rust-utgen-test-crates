pub const fn complement(&self) -> Self {
        let mask = [!self.mask[0], !self.mask[1], !self.mask[2], !self.mask[3]];
        AsciiSet { mask }
    }