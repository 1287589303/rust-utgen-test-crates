// Answer 0

#[test]
fn test_put_with_integer() {
    struct SimplePool<T>(Vec<T>);
    
    impl<T: Send> SimplePool<T> {
        fn new() -> Self {
            SimplePool(Vec::new())
        }
        
        fn put(&mut self, item: T) {
            self.0.push(item);
        }
    }
    
    let pool: SimplePool<i32> = SimplePool::new();
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(1)),
    };
    PoolGuard::put(guard);
}

#[test]
fn test_put_with_string() {
    struct SimplePool<T>(Vec<T>);
    
    impl<T: Send> SimplePool<T> {
        fn new() -> Self {
            SimplePool(Vec::new())
        }
        
        fn put(&mut self, item: T) {
            self.0.push(item);
        }
    }
    
    let pool: SimplePool<String> = SimplePool::new();
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(String::from("test"))),
    };
    PoolGuard::put(guard);
}

#[test]
fn test_put_with_user_defined_struct() {
    #[derive(Debug)]
    struct MyStruct {
        value: i32,
    }
    
    impl Send for MyStruct {}
    
    struct SimplePool<T>(Vec<T>);
    
    impl<T: Send> SimplePool<T> {
        fn new() -> Self {
            SimplePool(Vec::new())
        }
        
        fn put(&mut self, item: T) {
            self.0.push(item);
        }
    }
    
    let pool: SimplePool<MyStruct> = SimplePool::new();
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(MyStruct { value: 42 })),
    };
    PoolGuard::put(guard);
}

#[test]
fn test_put_with_empty_pool() {
    struct SimplePool<T>(Vec<T>);
    
    impl<T: Send> SimplePool<T> {
        fn new() -> Self {
            SimplePool(Vec::new())
        }
        
        fn put(&mut self, item: T) {
            self.0.push(item);
        }
    }
    
    let pool: SimplePool<i32> = SimplePool::new();
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(0)),
    };
    PoolGuard::put(guard);
}

#[test]
fn test_put_with_full_pool() {
    struct SimplePool<T>(Vec<T>);
    
    impl<T: Send> SimplePool<T> {
        fn new() -> Self {
            SimplePool(Vec::new())
        }
        
        fn put(&mut self, item: T) {
            self.0.push(item);
        }
    }
    
    let mut pool: SimplePool<i32> = SimplePool(vec![1, 2, 3]);
    let guard = PoolGuard {
        pool: &pool,
        value: Some(Box::new(4)),
    };
    PoolGuard::put(guard);
}

