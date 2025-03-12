pub fn len(&self) -> usize {
        // This should never panic since deserialization checks that the
        // length can fit into a usize.
        usize::try_from(self.accels.as_ref()[0]).unwrap()
    }