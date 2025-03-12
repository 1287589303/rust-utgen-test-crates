fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {
        ByteClassElementRanges { elements: self.elements(class), range: None }
    }