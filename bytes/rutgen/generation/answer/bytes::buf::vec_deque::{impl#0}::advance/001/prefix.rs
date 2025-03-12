// Answer 0

#[test]
fn test_advance_zero() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);
    deque.advance(0);
}

#[test]
fn test_advance_equal_to_length() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);
    deque.advance(4);
}

#[test]
fn test_advance_one() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);
    deque.advance(1);
}

#[test]
fn test_advance_exceed_length() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);
    deque.advance(5);
}

#[test]
fn test_advance_one_exceed_length() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);
    deque.advance(6);
}

#[test]
fn test_advance_empty_deque() {
    let mut deque: VecDeque<u8> = VecDeque::new();
    deque.advance(0);
}

#[test]
fn test_advance_empty_deque_exceed() {
    let mut deque: VecDeque<u8> = VecDeque::new();
    deque.advance(1);
}

