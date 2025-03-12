// Answer 0

#[test]
fn test_size_hint_empty() {
    struct CaptureNamesTest;
    
    impl Iterator for CaptureNamesTest {
        type Item = Option<&'static str>;
        
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(0))
        }
    }
    
    let mut capture_names = CaptureNamesTest;
    capture_names.size_hint();
}

#[test]
fn test_size_hint_one() {
    struct CaptureNamesTest {
        called: bool,
    }
    
    impl Iterator for CaptureNamesTest {
        type Item = Option<&'static str>;
        
        fn next(&mut self) -> Option<Self::Item> {
            if !self.called {
                self.called = true;
                Some(Some("group1"))
            } else {
                None
            }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (1, Some(1))
        }
    }
    
    let mut capture_names = CaptureNamesTest { called: false };
    capture_names.size_hint();
}

#[test]
fn test_size_hint_multiple() {
    struct CaptureNamesTest {
        count: usize,
    }
    
    impl Iterator for CaptureNamesTest {
        type Item = Option<&'static str>;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                let name = format!("group{}", self.count + 1);
                self.count += 1;
                Some(Some(Box::leak(name.into_boxed_str())))
            } else {
                None
            }
        }
        
        fn size_hint(&self) -> (usize, Option<usize>) {
            (3, Some(3))
        }
    }
    
    let mut capture_names = CaptureNamesTest { count: 0 };
    capture_names.size_hint();
}

