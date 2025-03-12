pub fn is_tuple(&self) -> bool {
        matches!(*self, Origin::Tuple(..))
    }