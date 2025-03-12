// Answer 0

#[test]
fn test_cross_forward_with_exact_literals() {
    let mut seq1 = Seq::new(vec![ 
        Literal::exact("foo".as_bytes()), 
        Literal::exact("bar".as_bytes())
    ]);
    let mut seq2 = Seq::new(vec![ 
        Literal::exact("baz".as_bytes()),
        Literal::exact("quux".as_bytes())
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_exact_and_inexact_literals() {
    let mut seq1 = Seq::new(vec![ 
        Literal::exact("foo".as_bytes()), 
        Literal::inexact("bar".as_bytes())
    ]);
    let mut seq2 = Seq::new(vec![ 
        Literal::exact("baz".as_bytes()),
        Literal::inexact("quux".as_bytes())
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_all_exact_literals() {
    let mut seq1 = Seq::new(vec![ 
        Literal::exact("apple".as_bytes()), 
        Literal::exact("banana".as_bytes())
    ]);
    let mut seq2 = Seq::new(vec![ 
        Literal::exact("cherry".as_bytes()),
        Literal::exact("date".as_bytes())
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_inexact_other_to_exact_self() {
    let mut seq1 = Seq::new(vec![ 
        Literal::exact("hello".as_bytes()), 
        Literal::exact("world".as_bytes())
    ]);
    let mut seq2 = Seq::new(vec![ 
        Literal::inexact("example".as_bytes()),
        Literal::exact("test".as_bytes())
    ]);
    seq1.cross_forward(&mut seq2);
}

#[test]
fn test_cross_forward_with_exact_self_and_empty_inexact_other() {
    let mut seq1 = Seq::new(vec![ 
        Literal::exact("start".as_bytes()), 
        Literal::exact("middle".as_bytes())
    ]);
    let mut seq2 = Seq::new(vec![ 
        Literal::inexact("".as_bytes())
    ]);
    seq1.cross_forward(&mut seq2);
}

