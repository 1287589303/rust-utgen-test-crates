pub fn get(this: &Lazy<T, F>) -> &T {
        this.0.get()
    }