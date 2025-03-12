fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.pool.put_value(value);
        }
    }