fn into_iter(self) -> Self::IntoIter {
        RcVecIntoIter {
            inner: self.inner.into_iter(),
        }
    }