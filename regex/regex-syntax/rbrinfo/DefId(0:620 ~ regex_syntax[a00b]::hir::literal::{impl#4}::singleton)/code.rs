pub fn singleton(lit: Literal) -> Seq {
        Seq { literals: Some(vec![lit]) }
    }