pub fn elements(&self, class: Unit) -> ByteClassElements {
        ByteClassElements { classes: self, class, byte: 0 }
    }