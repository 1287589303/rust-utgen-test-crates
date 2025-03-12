fn put_value(&self, value: Box<T>) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(value);
    }