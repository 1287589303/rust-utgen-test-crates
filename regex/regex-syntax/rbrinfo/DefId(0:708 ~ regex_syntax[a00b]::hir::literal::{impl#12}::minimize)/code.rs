fn minimize(literals: &mut Vec<Literal>, keep_exact: bool) {
        let mut trie = PreferenceTrie {
            states: vec![],
            matches: vec![],
            next_literal_index: 1,
        };
        let mut make_inexact = vec![];
        literals.retain_mut(|lit| match trie.insert(lit.as_bytes()) {
            Ok(_) => true,
            Err(i) => {
                if !keep_exact {
                    make_inexact.push(i.checked_sub(1).unwrap());
                }
                false
            }
        });
        for i in make_inexact {
            literals[i].make_inexact();
        }
    }