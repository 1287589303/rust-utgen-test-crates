fn try_get_int(&mut self, nbytes: usize) -> Result<i64, TryGetError> {
            (**self).try_get_int(nbytes)
        }