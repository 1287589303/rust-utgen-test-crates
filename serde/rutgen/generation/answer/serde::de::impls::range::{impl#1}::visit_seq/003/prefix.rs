// Answer 0

#[test]
fn test_visit_seq_valid_case() {
    struct TestSeq {
        values: Vec<Option<i64>>,
        index: usize,
    }

    impl SeqAccess<'static> for TestSeq {
        type Error = ();
        
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'static>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index].take();
                self.index += 1;
                Ok(value)
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq { values: vec![Some(10), Some(20)], index: 0 };
    let visitor = RangeVisitor::<i64> { expecting: "a range", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_missing_start() {
    struct TestSeq {
        values: Vec<Option<i64>>,
        index: usize,
    }

    impl SeqAccess<'static> for TestSeq {
        type Error = ();
        
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'static>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index].take();
                self.index += 1;
                Ok(value)
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq { values: vec![None, Some(20)], index: 0 };
    let visitor = RangeVisitor::<i64> { expecting: "a range", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

#[test]
fn test_visit_seq_missing_end() {
    struct TestSeq {
        values: Vec<Option<i64>>,
        index: usize,
    }

    impl SeqAccess<'static> for TestSeq {
        type Error = ();
        
        fn next_element<T>(&mut self) -> Result<Option<T>, Self::Error>
        where
            T: Deserialize<'static>,
        {
            if self.index < self.values.len() {
                let value = self.values[self.index].take();
                self.index += 1;
                Ok(value)
            } else {
                Ok(None)
            }
        }
    }

    let seq = TestSeq { values: vec![Some(10), None], index: 0 };
    let visitor = RangeVisitor::<i64> { expecting: "a range", phantom: std::marker::PhantomData };
    let _ = visitor.visit_seq(seq);
}

