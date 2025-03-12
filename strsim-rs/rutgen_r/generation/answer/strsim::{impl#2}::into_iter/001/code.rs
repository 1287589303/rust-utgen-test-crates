// Answer 0

#[test]
fn test_into_iter_empty_string() {
    struct Wrapper(String);
    impl Wrapper {
        fn into_iter(self) -> impl Iterator<Item = char> {
            self.0.chars()
        }
    }
    
    let wrapper = Wrapper("".to_string());
    let mut iter = wrapper.into_iter();
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_single_character() {
    struct Wrapper(String);
    impl Wrapper {
        fn into_iter(self) -> impl Iterator<Item = char> {
            self.0.chars()
        }
    }
    
    let wrapper = Wrapper("a".to_string());
    let mut iter = wrapper.into_iter();
    assert_eq!(iter.next(), Some('a'));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_multiple_characters() {
    struct Wrapper(String);
    impl Wrapper {
        fn into_iter(self) -> impl Iterator<Item = char> {
            self.0.chars()
        }
    }
    
    let wrapper = Wrapper("abc".to_string());
    let mut iter = wrapper.into_iter();
    assert_eq!(iter.next(), Some('a'));
    assert_eq!(iter.next(), Some('b'));
    assert_eq!(iter.next(), Some('c'));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_into_iter_unicode_characters() {
    struct Wrapper(String);
    impl Wrapper {
        fn into_iter(self) -> impl Iterator<Item = char> {
            self.0.chars()
        }
    }
    
    let wrapper = Wrapper("αβγ".to_string());
    let mut iter = wrapper.into_iter();
    assert_eq!(iter.next(), Some('α'));
    assert_eq!(iter.next(), Some('β'));
    assert_eq!(iter.next(), Some('γ'));
    assert_eq!(iter.next(), None);
}

